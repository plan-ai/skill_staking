import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SkillStaking } from "../target/types/skill_staking";
import { PublicKey } from "@saberhq/solana-contrib";
import * as ed from "@noble/ed25519";
import { rpcConfig } from "./rpcConfig";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("skill_staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SkillStaking as Program<SkillStaking>;
  const {
    provider: { connection },
  } = program;
  const { web3 } = anchor;
  const signatureVersion = 1;
  const signingName = "defios.com";
  const userName: string = "sunguru98";
  const userPubkey = new PublicKey(
    "81sWMLg1EgYps3nMwyeSW1JfjKgFqkGYPP85vTnkFzRn"
  );
  const testMetadataLink = "https://github.com/defi-os/defios-rust-core.ts";
  const stakeAmount = 1;
  async function create_keypair() {
    const keypair = web3.Keypair.generate();
    await connection.confirmTransaction(
      {
        signature: await connection.requestAirdrop(
          keypair.publicKey,
          web3.LAMPORTS_PER_SOL
        ),
        ...(await connection.getLatestBlockhash()),
      },
      "confirmed"
    );
    return keypair;
  }

  async function get_pda_from_seeds(seeds) {
    return await web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );
  }

  //main testsuite code
  async function create_name_router() {
    //generating keypair and airdropping solana to it
    const routerCreatorKeypair = await create_keypair();

    //get public key of pda ideally generated using seeds
    const [nameRouterAccount] = await get_pda_from_seeds([
      Buffer.from(signingName),
      Buffer.from(signatureVersion.toString()),
      routerCreatorKeypair.publicKey.toBuffer(),
    ]);

    //call create name router function
    await program.methods
      .createNameRouter(signingName, signatureVersion)
      .accounts({
        nameRouterAccount,
        routerCreator: routerCreatorKeypair.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([routerCreatorKeypair])
      .rpc(rpcConfig);
    return [routerCreatorKeypair, nameRouterAccount];
  }

  async function create_verified_user(
    routerCreatorKeypair,
    nameRouterAccount,
    pubKey
  ) {
    // Signature test
    //Create byte array of message
    const message = Uint8Array.from(
      Buffer.from(`DefiOS(${userName}, ${userPubkey.toString()})`)
    );

    //create signature from message and secret key
    const signature = await ed.sign(
      message,
      routerCreatorKeypair.secretKey.slice(0, 32)
    );

    //create instruction from message, public key, and signature of account
    const createED25519Ix = web3.Ed25519Program.createInstructionWithPublicKey({
      message: message,
      publicKey: routerCreatorKeypair.publicKey.toBytes(),
      signature,
    });

    //gets public key from seeds
    const [verifiedUserAccount] = await get_pda_from_seeds([
      Buffer.from(userName),
      pubKey.toBuffer(),
      nameRouterAccount.toBuffer(),
    ]);

    //calls add verified user method
    await program.methods
      .addVerifiedUser(
        userName,
        pubKey,
        Buffer.from(message),
        Buffer.from(signature)
      )
      .accounts({
        nameRouterAccount,
        verifiedUserAccount,
        routerCreator: routerCreatorKeypair.publicKey,
        sysvarInstructions: web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([routerCreatorKeypair])
      .preInstructions([createED25519Ix])
      .rpc(rpcConfig);
    return [verifiedUserAccount];
  }
  it("Create Name Router", async () => {
    // Add your test here.
    const [routerCreatorKeypair, nameRouterAccount] =
      await create_name_router();
    //get data related to name router pda
    const {
      routerCreator,
      signatureVersion: fSignatureVersion,
      signingDomain,
      bump,
      totalVerifiedUsers,
    } = await program.account.nameRouter.fetch(nameRouterAccount);
  });

  it("Adds a verified user", async () => {
    const [routerCreatorKeypair, nameRouterAccount] =
      await create_name_router();

    const [verifiedUserAccount] = await create_verified_user(
      routerCreatorKeypair,
      nameRouterAccount,
      userPubkey
    );
  });

  it("Add freelacer", async () => {
    const [routerCreatorKeypair, nameRouterAccount] =
      await create_name_router();

    const freelancer = await create_keypair();
    const [verifiedUserAccount] = await create_verified_user(
      routerCreatorKeypair,
      nameRouterAccount,
      freelancer.publicKey
    );

    const [freelanceAccount] = await get_pda_from_seeds([
      Buffer.from("freelance"),
      freelancer.publicKey.toBuffer(),
    ]);

    await program.methods
      .addFreelancer(testMetadataLink)
      .accounts({
        freelancer: freelancer.publicKey,
        freelancerVerifiedUser: verifiedUserAccount,
        freelanceAccount: freelanceAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([freelancer])
      .rpc(rpcConfig);
  });
  it("Create Bounty", async () => {
    const bountyCreator = await create_keypair();
    const mintAuthority = await create_keypair();
    const mintAddress = await createMint(
      connection,
      mintAuthority,
      mintAuthority.publicKey,
      mintAuthority.publicKey,
      6
    );
    const bountyCreatorTokenAddress = await createAssociatedTokenAccount(
      connection,
      bountyCreator,
      mintAddress,
      bountyCreator.publicKey
    );
    const [bounty] = await get_pda_from_seeds([
      Buffer.from("bounty"),
      bountyCreator.publicKey.toBuffer(),
      Buffer.from("1"),
    ]);

    const bountyTokenAddress = await getAssociatedTokenAddress(
      mintAddress,
      bounty,
      true
    );
    await mintTo(
      connection,
      bountyCreator,
      mintAddress,
      bountyCreatorTokenAddress,
      mintAuthority,
      stakeAmount
    );

    await program.methods
      .createBounty("1", new anchor.BN(stakeAmount), testMetadataLink, [], null)
      .accounts({
        bountyCreator: bountyCreator.publicKey,
        bountyCreatorTokenAccount: bountyCreatorTokenAddress,
        bountyAccount: bounty,
        bountyTokenAccount: bountyTokenAddress,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        usdcMint: mintAddress,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([bountyCreator])
      .rpc(rpcConfig);
  });
  it("Apply bounty", async () => {
    const [routerCreatorKeypair, nameRouterAccount] =
      await create_name_router();

    const freelancer = await create_keypair();
    const [verifiedUserAccount] = await create_verified_user(
      routerCreatorKeypair,
      nameRouterAccount,
      freelancer.publicKey
    );

    const [freelanceAccount] = await get_pda_from_seeds([
      Buffer.from("freelance"),
      freelancer.publicKey.toBuffer(),
    ]);

    await program.methods
      .addFreelancer(testMetadataLink)
      .accounts({
        freelancer: freelancer.publicKey,
        freelancerVerifiedUser: verifiedUserAccount,
        freelanceAccount: freelanceAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([freelancer])
      .rpc(rpcConfig);

    const bountyCreator = await create_keypair();
    const mintAuthority = await create_keypair();
    const mintAddress = await createMint(
      connection,
      mintAuthority,
      mintAuthority.publicKey,
      mintAuthority.publicKey,
      6
    );
    const bountyCreatorTokenAddress = await createAssociatedTokenAccount(
      connection,
      bountyCreator,
      mintAddress,
      bountyCreator.publicKey
    );
    const [bounty] = await get_pda_from_seeds([
      Buffer.from("bounty"),
      bountyCreator.publicKey.toBuffer(),
      Buffer.from("1"),
    ]);

    const bountyTokenAddress = await getAssociatedTokenAddress(
      mintAddress,
      bounty,
      true
    );
    await mintTo(
      connection,
      bountyCreator,
      mintAddress,
      bountyCreatorTokenAddress,
      mintAuthority,
      stakeAmount
    );

    await program.methods
      .createBounty("1", new anchor.BN(stakeAmount), testMetadataLink, [], null)
      .accounts({
        bountyCreator: bountyCreator.publicKey,
        bountyCreatorTokenAccount: bountyCreatorTokenAddress,
        bountyAccount: bounty,
        bountyTokenAccount: bountyTokenAddress,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        usdcMint: mintAddress,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([bountyCreator])
      .rpc(rpcConfig);

    await program.methods
      .applyBounty()
      .accounts({
        freelancer: freelancer.publicKey,
        freelanceAccount: freelanceAccount,
        bountyAccount: bounty,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([freelancer])
      .rpc(rpcConfig);
  });
});
