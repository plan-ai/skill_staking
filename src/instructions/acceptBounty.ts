/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category AcceptBounty
 * @category generated
 */
export const acceptBountyStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */
}>(
  [['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)]],
  'AcceptBountyInstructionArgs'
)
/**
 * Accounts required by the _acceptBounty_ instruction
 *
 * @property [**signer**] firstSigner
 * @property [**signer**] secondSigner
 * @property [] freelanceAccount
 * @property [] multiSig
 * @property [_writable_] bountyAccount
 * @category Instructions
 * @category AcceptBounty
 * @category generated
 */
export type AcceptBountyInstructionAccounts = {
  firstSigner: web3.PublicKey
  secondSigner: web3.PublicKey
  freelanceAccount: web3.PublicKey
  multiSig: web3.PublicKey
  bountyAccount: web3.PublicKey
  systemProgram?: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const acceptBountyInstructionDiscriminator = [
  165, 37, 99, 130, 123, 244, 67, 35,
]

/**
 * Creates a _AcceptBounty_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category AcceptBounty
 * @category generated
 */
export function createAcceptBountyInstruction(
  accounts: AcceptBountyInstructionAccounts,
  programId = new web3.PublicKey('CnMMyfQSGk7h6YNf2uLmBuLpfBKuMTYPct6PmFMM3P24')
) {
  const [data] = acceptBountyStruct.serialize({
    instructionDiscriminator: acceptBountyInstructionDiscriminator,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.firstSigner,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.secondSigner,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.freelanceAccount,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.multiSig,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.bountyAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc)
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}