const PROGRAM_ID = 'CnMMyfQSGk7h6YNf2uLmBuLpfBKuMTYPct6PmFMM3P24';

const path = require('path');
const programDir = path.join(__dirname, 'programs','skill_staking');
const idlDir = path.join(__dirname,'target','idl');
const sdkDir = path.join(__dirname,'src');
const PROGRAM_NAME = "skill_staking";
const binaryInstallDir = path.join(__dirname,'target');

module.exports = {
  idlGenerator: 'anchor',
  programName: PROGRAM_NAME,
  programId: PROGRAM_ID,
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
