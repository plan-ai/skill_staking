export * from './Bounty'
export * from './Freelancer'
export * from './Multisig'
export * from './NameRouter'
export * from './SkillStake'
export * from './VerifiedUser'

import { NameRouter } from './NameRouter'
import { VerifiedUser } from './VerifiedUser'
import { Bounty } from './Bounty'
import { Freelancer } from './Freelancer'
import { SkillStake } from './SkillStake'
import { Multisig } from './Multisig'

export const accountProviders = {
  NameRouter,
  VerifiedUser,
  Bounty,
  Freelancer,
  SkillStake,
  Multisig,
}
