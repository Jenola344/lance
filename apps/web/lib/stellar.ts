import { StellarWalletsKit, Networks } from "@creit.tech/stellar-wallets-kit";

// TODO: See docs/ISSUES.md — "Wallet Connection"
let kit: StellarWalletsKit | null = null;

export function getWalletsKit(): StellarWalletsKit {
  if (!kit) {
    kit = new StellarWalletsKit({
      network:
        (process.env.NEXT_PUBLIC_STELLAR_NETWORK as Networks) ??
        Networks.TESTNET,
      selectedWalletId: "freighter",
    });
  }
  return kit;
}

/** Opens wallet select modal and returns the connected public key. */
export async function connectWallet(): Promise<string> {
  return "GD...FAKE_ADDRESS";
}

/** Signs an XDR transaction string via the connected wallet. */
export async function signTransaction(xdr: string): Promise<string> {
  return "FAKE_SIGNATURE";
}
