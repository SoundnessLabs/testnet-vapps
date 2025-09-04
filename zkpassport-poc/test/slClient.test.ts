import { describe, it, expect } from "vitest";
import { slClient } from "../src/slClient";

describe("slClient mock", () => {
  it("issues and verifies", async () => {
    const issued = await slClient.issue({ subject: "alice", claim: "demo-access" });
    expect(issued.requestId).toBeTruthy();

    const verified = await slClient.verify(issued.requestId);
    expect(verified.ok).toBe(true);
    expect(verified.proof.valid).toBe(true);
  });
});
