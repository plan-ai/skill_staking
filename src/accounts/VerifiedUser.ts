/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'

/**
 * Arguments used to create {@link VerifiedUser}
 * @category Accounts
 * @category generated
 */
export type VerifiedUserArgs = {
  bump: number
  nameRouter: web3.PublicKey
  userName: string
  userPubkey: web3.PublicKey
}

export const verifiedUserDiscriminator = [197, 144, 184, 72, 82, 65, 99, 144]
/**
 * Holds the data for the {@link VerifiedUser} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class VerifiedUser implements VerifiedUserArgs {
  private constructor(
    readonly bump: number,
    readonly nameRouter: web3.PublicKey,
    readonly userName: string,
    readonly userPubkey: web3.PublicKey
  ) {}

  /**
   * Creates a {@link VerifiedUser} instance from the provided args.
   */
  static fromArgs(args: VerifiedUserArgs) {
    return new VerifiedUser(
      args.bump,
      args.nameRouter,
      args.userName,
      args.userPubkey
    )
  }

  /**
   * Deserializes the {@link VerifiedUser} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [VerifiedUser, number] {
    return VerifiedUser.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link VerifiedUser} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<VerifiedUser> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find VerifiedUser account at ${address}`)
    }
    return VerifiedUser.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      'CnMMyfQSGk7h6YNf2uLmBuLpfBKuMTYPct6PmFMM3P24'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, verifiedUserBeet)
  }

  /**
   * Deserializes the {@link VerifiedUser} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [VerifiedUser, number] {
    return verifiedUserBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link VerifiedUser} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return verifiedUserBeet.serialize({
      accountDiscriminator: verifiedUserDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link VerifiedUser} for the provided args.
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   */
  static byteSize(args: VerifiedUserArgs) {
    const instance = VerifiedUser.fromArgs(args)
    return verifiedUserBeet.toFixedFromValue({
      accountDiscriminator: verifiedUserDiscriminator,
      ...instance,
    }).byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link VerifiedUser} data from rent
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    args: VerifiedUserArgs,
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      VerifiedUser.byteSize(args),
      commitment
    )
  }

  /**
   * Returns a readable version of {@link VerifiedUser} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      bump: this.bump,
      nameRouter: this.nameRouter.toBase58(),
      userName: this.userName,
      userPubkey: this.userPubkey.toBase58(),
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const verifiedUserBeet = new beet.FixableBeetStruct<
  VerifiedUser,
  VerifiedUserArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['bump', beet.u8],
    ['nameRouter', beetSolana.publicKey],
    ['userName', beet.utf8String],
    ['userPubkey', beetSolana.publicKey],
  ],
  VerifiedUser.fromArgs,
  'VerifiedUser'
)
