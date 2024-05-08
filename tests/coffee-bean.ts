import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { CoffeeBean } from "../target/types/coffee_bean";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getMint, getAccount, Mint, Account, getAssociatedTokenAddressSync } from "@solana/spl-token";

import {
  LAMPORTS_PER_SOL,
  PublicKey,
  Keypair,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
  clusterApiUrl,
  Connection
} from "@solana/web3.js";


const GLOBAL_STATE_SEED = "GLOBAL_STATE_SEED";
const VAULT_SEED = "VAULT_SEED";
const USER_STATE_SEED = "USER_STATE_SEED";
const TOKENMINT = new anchor.web3.PublicKey("EUdP7XScSit96rdZGALuBAendca1Vtkgu3kBANV81DNv");

const systemProgram = anchor.web3.SystemProgram.programId;
const tokenProgram = TOKEN_PROGRAM_ID;
const associatedTokenProgram = ASSOCIATED_TOKEN_PROGRAM_ID
const rent = anchor.web3.SYSVAR_RENT_PUBKEY;
const clock = anchor.web3.SYSVAR_CLOCK_PUBKEY;



const delay = (delayInms) => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(1);
    }, delayInms);
  });
}

export const pda = (
  seeds: (Buffer | Uint8Array)[],
  programId: anchor.web3.PublicKey
): anchor.web3.PublicKey => {
  const [pdaKey] = anchor.web3.PublicKey.findProgramAddressSync(
    seeds,
    programId
  );
  return pdaKey;
}

describe("coffee-bean", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CoffeeBean as Program<CoffeeBean>;

  it("Is initialized!", async () => {
    const ADMINTREASURY = getAssociatedTokenAddressSync(TOKENMINT, provider.wallet.publicKey);
    console.log("admin------->", provider.wallet.publicKey);
    console.log("admin treasury------->", ADMINTREASURY);
    // Add your test here.
    console.log("program id ---------->", program.programId.toBase58());

    const globalStateKey = await pda([Buffer.from(GLOBAL_STATE_SEED)], program.programId);
    console.log("global state --------->", globalStateKey.toBase58());
    const vaultKey = await pda([Buffer.from(VAULT_SEED)], program.programId);
    console.log("vault------->", vaultKey.toBase58());
    const admin = provider.wallet;
    const txid = await program.methods
      .initialize(admin.publicKey)
      .accounts({
        authority: admin.publicKey,
        globalState: globalStateKey,
        tokenMint: TOKENMINT,
        treasury: admin.publicKey,
        vault: vaultKey,
        tokenProgram,
        systemProgram,
        rent,
        clock
      })
      .rpc({ skipPreflight: true });
      console.log("transaction id->",txid)
  
});

  // it("buy and hatch eggs", async () => {
  //   const globalStateKey = await pda([Buffer.from(GLOBAL_STATE_SEED)], program.programId);
  //   console.log(globalStateKey.toBase58());
  //   const vaultKey = await pda([Buffer.from(VAULT_SEED)], program.programId);
  //   console.log(vaultKey.toBase58());
  //   const userStateKey = await pda([Buffer.from(USER_STATE_SEED), user.publicKey.toBuffer()], program.programId);
  //   const adminUserStateKey = await pda([Buffer.from(USER_STATE_SEED), admin.publicKey.toBuffer()], program.programId);
  //   let globalData = await program.account.globalState.fetch(globalStateKey);
  //   const tx = new Transaction().add( 
  //     await program.methods
  //       .buyEggs(new anchor.BN(5).mul(new anchor.BN(LAMPORTS_PER_SOL)))
  //       .accounts({
  //         user: user.publicKey,
  //         globalState: globalStateKey,
  //         treasury: globalData.treasury,
  //         vault: vaultKey,
  //         userState: userStateKey,
  //         systemProgram: SystemProgram.programId,
  //         rent: SYSVAR_RENT_PUBKEY
  //       })
  //       .instruction()
  //   );
  //   tx.add(
  //     await program.methods
  //       .hatchEggs()
  //       .accounts({
  //         user: user.publicKey,
  //         globalState: globalStateKey,
  //         vault: vaultKey,
  //         userState: userStateKey,
  //         referral: admin.publicKey,
  //         referralState: adminUserStateKey,
  //       })
  //       .instruction()
  //   );
  //   ///let simulRes = await provider.simulate(tx, [user]);
  //   ///console.log('simulRes =', simulRes);

  //   let txHash = await sendAndConfirmTransaction(provider.connection, tx, [user]);
  //   console.log("Your transaction signature", txHash);
  //   let solBal = await provider.connection.getBalance(user.publicKey);
  //   console.log(solBal);

  //   let userStateData = await program.account.userState.fetch(userStateKey);
  //   console.log("userStateData.miners", userStateData.miners.toNumber());
  // })

  // it("sell eggs", async () => {
  //   await delay(2000);
  //   const globalStateKey = await pda([Buffer.from(GLOBAL_STATE_SEED)], program.programId);
  //   console.log(globalStateKey.toBase58());
  //   const vaultKey = await pda([Buffer.from(VAULT_SEED)], program.programId);
  //   console.log(vaultKey.toBase58());
  //   const userStateKey = await pda([Buffer.from(USER_STATE_SEED), user.publicKey.toBuffer()], program.programId);
  //   const adminUserStateKey = await pda([Buffer.from(USER_STATE_SEED), admin.publicKey.toBuffer()], program.programId);
  //   let globalData = await program.account.globalState.fetch(globalStateKey);
  //   const tx = new Transaction().add(
  //     await program.methods
  //       .sellEggs()
  //       .accounts({
  //         user: user.publicKey,
  //         globalState: globalStateKey,
  //         treasury: globalData.treasury,
  //         vault: vaultKey,
  //         userState: userStateKey,
  //         systemProgram: SystemProgram.programId
  //       })
  //       .instruction()
  //   );
  //   let txHash = await sendAndConfirmTransaction(provider.connection, tx, [user]);
  //   console.log("Your transaction signature", txHash);
  //   let solBal = await provider.connection.getBalance(user.publicKey);
  //   console.log(solBal);
  // })
  // it("getInfo", async () => {
  //   let userStateKey = new PublicKey("AZ8Zjm3qBbxLXWdxu44LnkuYQ1MJdmjfFgoNcjWnZTqD");
  //   let connection = new Connection(clusterApiUrl("devnet"));
  //   let provider1 = new anchor.Provider(connection, provider.wallet, anchor.Provider.defaultOptions())
  //   const otherProgram = new anchor.Program(IDL, new PublicKey("557BPiUp8WSumh7PcLpXE12VZppRhiezdHRdfhKAVANn"), provider1);

  //   let userStateData = await otherProgram.account.userState.fetch(userStateKey);
  //   console.log("userStateData.miners", userStateData.miners.toNumber());
  // })
});

export const airdropSol = async (
  provider: anchor.Provider,
  target: anchor.web3.PublicKey,
  lamps: number
): Promise<string> => {
  const sig: string = await provider.connection.requestAirdrop(target, lamps);
  await provider.connection.confirmTransaction(sig);
  return sig;
};