// TODO: Soroban contract call helpers — see docs/ISSUES.md — "Contract Call Helpers"

/** Calls escrow.deposit — builds XDR, signs via Freighter, submits. */
export async function depositEscrow(_params: {
  jobId: bigint;
  clientAddress: string;
  freelancerAddress: string;
  amountUsdc: bigint;
  milestones: number;
}): Promise<string> {
  return "FAKE_TX_HASH_DEPOSIT";
}

/** Calls escrow.release_milestone */
export async function releaseMilestone(_jobId: bigint): Promise<string> {
  return "FAKE_TX_HASH_RELEASE";
}

/** Calls escrow.open_dispute */
export async function openDispute(_jobId: bigint): Promise<string> {
  return "FAKE_TX_HASH_DISPUTE";
}
