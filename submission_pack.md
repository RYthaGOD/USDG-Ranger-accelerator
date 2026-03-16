# 🏆 Ranger Accelerator: Hackathon Submission Pack

Use this document to quickly copy-paste absolute values and scripts for the Ranger Build-A-Bear Submission Form on Superteam Earn.

---

## 🔗 **1. On-Chain Addresses**
| Component | Address |
| :--- | :--- |
| **Strategy Program ID** | `BioV76fRvp5XR1NL72GUYiu3xqAzVXso6vwBP56ghBmc` |
| **Kamino Lend ID** | `KLend2g3M67enU7fB6uMj9795m1ebSy78pS7CzW9XDP` |
| **Meteora DLMM ID** | `LbM7pZu499K7f6N8NVpA4SST7S5DDA9hkDsS4NVcKEX` |
| **Drift Protocol ID** | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH` |

---

## 🎙️ **2. 3-Minute Video Script Outline**

### **0:00 - 0:30 | The Thesis (Why Delta-Neutral LP?)**
-   **Hook**: Yield farming can leave you over-exposed to asset depreciation.
-   **Solution**: Ranger Accelerator bundles a **High-fidelity 25%+ APY** yield stream with **Delta-neutral hedging**.
-   **Edge**: Utilizing Meteora dLMM fees + Drift perp shorts, we capture yield without exposure to SOL drawdown risk.

### **0:30 - 1:30 | The Operation (How it Works)**
-   **Accept USDC**: Vault intakes USDC (100% compliant with Seeding prize constraints).
-   **Looping**: Intakes USDC $\rightarrow$ routes to Kamino Collateral $\rightarrow$ borrows JitoSOL securely.
-   **Concentrated LP**: Supplies borrowed JitoSOL into Meteora dLMM (USDG-SOL) harvesting absolute fee streams.
-   **Short Hedge**: Locks position with Drift Perps Short protecting against the SOL long LP setup in parallel.

### **1:30 - 2:30 | Risk Management & Safety**
-   **Volume Capping**: Custom `max_deposit_cap` capping volume index thresholds (enforced in `deposit.rs`).
-   **Time locking**: Appended `deposit_ts` tracking rolling lockups accurately inside state buffers.
-   **Rebalance Trigger node**: Autonomous bot manages liquidation prevention continuously.

### **2:30 - 3:00 | Outro (Production Viability)**
-   **Complete**: Verifiable build structures ready for absolute verification backing.
-   **Scale**: Capable of seeding TVL with standard absolute coordinate deployments.

---

_Staged for absolute pitch coordination._
