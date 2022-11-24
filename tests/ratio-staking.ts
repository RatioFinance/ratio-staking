import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { before } from 'mocha';
import { RatioStaking } from "../target/types/ratio_staking";
import stakingTestSuite from "./ratio-staking-tests";
import { constants } from "./contstants";
import { BN } from "bn.js";
import { pda } from "./utils";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import ratioTestMintKeypair from "./keys/ratio_test_mint-keypair.json";

describe("ratio-staking", async function () {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RatioStaking as Program<RatioStaking>;

  before("Is initialized!", async function() {
    // Add your test here.
    /* const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx); */
    this.stakingProgram = program;
    this.exists = {
      stake: false
    };

    this.provider = anchor.AnchorProvider.env();
    this.connection = this.provider.connection;

    // main user
    this.wallet = this.provider.wallet as anchor.Wallet;
    this.publicKey = this.wallet.publicKey;
    this.payer = this.wallet.payer;

    // constant values
    this.constants = constants;

    this.mint = await createMint(
      this.connection,
      this.payer,
      this.publicKey,
      null,
      6,
      Keypair.fromSecretKey(Uint8Array.from(ratioTestMintKeypair))
    )

    // dynamic values
    this.total = { xnos: new BN(0), reflection: new BN(0), rate: constants.initialRate };
    this.users = { user1: null, user2: null, user3: null, user4: null, otherUsers: null };
    this.nodes = { node1: null, node2: null, otherNodes: null };
    this.balances = { user: 0, beneficiary: 0, vaultJob: 0, vaultStaking: 0, vaultRewards: 0, vaultPool: 0 };

    // token vaults public keys
    this.vaults = {};
    this.vaults.staking = await pda(
      [utf8.encode('vault'), this.mint.toBuffer(), this.publicKey.toBuffer()],
      this.stakingProgram.programId
    );

    // public keys to be used in the instructions
    this.accounts = {};
    this.accounts.systemProgram = anchor.web3.SystemProgram.programId;
    this.accounts.tokenProgram = TOKEN_PROGRAM_ID;
    this.accounts.stakingProgram = this.stakingProgram.programId;
    this.accounts.rent = anchor.web3.SYSVAR_RENT_PUBKEY;
    this.accounts.authority = this.publicKey;
    this.accounts.payer = this.publicKey;
    this.accounts.mint = this.mint;
    
    this.accounts.user = await createAssociatedTokenAccount(
      this.connection,
      this.payer,
      this.mint,
      this.publicKey
    );
    await mintTo(
      this.connection,
      this.payer,
      this.mint,
      this.accounts.user,
      this.payer,
      100_000_000_000
    );
    this.balances.user = 100_000_000_000;
    // create ata


    this.accounts.settings = await pda([utf8.encode('settings')], this.stakingProgram.programId);
    this.accounts.tokenAccount = this.accounts.user;
    this.accounts.stake = await pda(
      [utf8.encode('stake'), this.mint.toBuffer(), this.publicKey.toBuffer()],
      this.stakingProgram.programId
    );
  });

  describe('staking-test', stakingTestSuite);
});

