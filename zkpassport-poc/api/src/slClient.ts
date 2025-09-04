// Mock Soundness Layer client (replace later with real SDK)
export type IssueRequest = { subject: string; claim: string };
export type Proof = { requestId: string; proofId: string; valid: boolean };

export const slClient = {
  health: async () => ({ ok: true, network: "sl-testnet", mock: true }),
  issue: async (req: IssueRequest) => {
    return { requestId: `req_${Math.random().toString(36).slice(2)}`, status: "issued" };
  },
  verify: async (requestId: string) => {
    const proof: Proof = { requestId, proofId: `prf_${Date.now()}`, valid: true };
    return { ok: true, proof };
  }
};
