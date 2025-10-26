import { useState } from "react";
import { useWallet } from "./hooks/useWallet";
import { connectWallet } from "./util/wallet";
import { Client } from "../packages/codebit/src/index";

const CONTRACT_ID = "CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP";

function App() {
  const { address, signTransaction } = useWallet();
  const [balance, setBalance] = useState<string>("0");
  const [loading, setLoading] = useState<boolean>(false);
  const [recipient, setRecipient] = useState<string>("");
  const [amount, setAmount] = useState<string>("");
  const [message, setMessage] = useState<string>("");

  const handleConnect = async () => {
    try {
      const result = await connectWallet();
      console.log("Wallet conectada:", result);
      setTimeout(() => {
        window.location.reload();
      }, 1000);
    } catch (error) {
      console.error("Error conectando wallet:", error);
      alert(
        "No se pudo conectar la wallet. Asegúrate de tener Freighter instalado."
      );
    }
  };

  const handleGetBalance = async () => {
    if (!address) {
      alert("Conectá tu wallet primero");
      return;
    }

    setLoading(true);

    try {
      const client = new Client({
        contractId: CONTRACT_ID,
        networkPassphrase: "Test SDF Network ; September 2015",
        rpcUrl: "https://soroban-testnet.stellar.org",
        publicKey: address,
      });

      const result = await client.balance({ account: address });
      setBalance(String(result));
    } catch (error) {
      console.error("Error:", error);
      setBalance("0");
    } finally {
      setLoading(false);
    }
  };

  const handleTransfer = async () => {
    if (!recipient || !amount) {
      setMessage("⚠️ Completá todos los campos");
      return;
    }

    if (!recipient.startsWith("G") || recipient.length !== 56) {
      setMessage(
        "⚠️ Dirección inválida (debe empezar con G y tener 56 caracteres)"
      );
      return;
    }

    if (!signTransaction) {
      setMessage("⚠️ Wallet no conectada correctamente");
      return;
    }

    setLoading(true);
    setMessage("");

    try {
      const client = new Client({
        contractId: CONTRACT_ID,
        networkPassphrase: "Test SDF Network ; September 2015",
        rpcUrl: "https://soroban-testnet.stellar.org",
        publicKey: address!,
      });

      const amountInTokens = BigInt(amount);

      const tx = await client.transfer({
        from: address!,
        to: recipient,
        amount: amountInTokens,
      });

      const result = await tx.signAndSend({ signTransaction });

      console.log("Transfer exitoso:", result);
      setMessage("✅ Transferencia exitosa!");
      setRecipient("");
      setAmount("");

      setTimeout(() => void handleGetBalance(), 2000);
    } catch (error) {
      console.error("Error:", error);
      setMessage("❌ Error en la transferencia");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      style={{
        padding: "40px",
        maxWidth: "600px",
        margin: "0 auto",
        fontFamily: "Arial, sans-serif",
        background: "linear-gradient(135deg, #2563eb 0%, #14b8a6 100%)",
        minHeight: "100vh",
      }}
    >
      <div
        style={{
          backgroundColor: "white",
          padding: "40px",
          borderRadius: "16px",
          boxShadow: "0 10px 30px rgba(0,0,0,0.2)",
        }}
      >
        <h1
          style={{
            background: "linear-gradient(135deg, #2563eb 0%, #14b8a6 100%)",
            WebkitBackgroundClip: "text",
            WebkitTextFillColor: "transparent",
            backgroundClip: "text",
            marginBottom: "10px",
            fontSize: "32px",
            textAlign: "center",
            fontWeight: "bold",
          }}
        >
          Buen Día Builders 🦈
        </h1>
        <p
          style={{
            textAlign: "center",
            color: "#666",
            fontSize: "14px",
            marginBottom: "30px",
          }}
        >
          Token CODEBIT en Stellar Testnet
        </p>

        {!address ? (
          <div>
            <p
              style={{
                fontSize: "18px",
                color: "#333",
                textAlign: "center",
                marginBottom: "20px",
                lineHeight: "1.6",
              }}
            >
              ¡Bienvenido al ecosistema CODEBIT!
              <br />
              <span style={{ fontSize: "16px", color: "#666" }}>
                Conectá tu wallet Freighter para comenzar a explorar tu balance
                de tokens
              </span>
            </p>
            <div style={{ marginTop: "30px", textAlign: "center" }}>
              <button
                type="button"
                onClick={() => void handleConnect()}
                style={{
                  padding: "16px 32px",
                  fontSize: "18px",
                  fontWeight: "bold",
                  background:
                    "linear-gradient(135deg, #2563eb 0%, #14b8a6 100%)",
                  color: "white",
                  border: "none",
                  borderRadius: "12px",
                  cursor: "pointer",
                  transition: "all 0.3s ease",
                  boxShadow: "0 4px 15px rgba(37, 99, 235, 0.4)",
                }}
                onMouseOver={(e) =>
                  (e.currentTarget.style.transform = "scale(1.05)")
                }
                onMouseOut={(e) =>
                  (e.currentTarget.style.transform = "scale(1)")
                }
              >
                🔗 Conectar Freighter Wallet
              </button>
            </div>
          </div>
        ) : (
          <div>
            <div
              style={{
                padding: "20px",
                background: "linear-gradient(135deg, #2563eb 0%, #14b8a6 100%)",
                borderRadius: "12px",
                marginBottom: "25px",
                color: "white",
              }}
            >
              <p
                style={{
                  fontWeight: "bold",
                  margin: "0 0 10px 0",
                  fontSize: "16px",
                }}
              >
                ✅ Wallet Conectada
              </p>
              <code
                style={{
                  backgroundColor: "rgba(255,255,255,0.2)",
                  padding: "10px",
                  borderRadius: "8px",
                  display: "block",
                  wordBreak: "break-all",
                  fontSize: "12px",
                  color: "white",
                  fontFamily: "monospace",
                }}
              >
                {address}
              </code>
            </div>

            <div style={{ marginBottom: "25px", textAlign: "center" }}>
              <button
                type="button"
                onClick={() => void handleGetBalance()}
                disabled={loading}
                style={{
                  padding: "14px 28px",
                  fontSize: "16px",
                  fontWeight: "bold",
                  backgroundColor: loading ? "#ccc" : "#10b981",
                  color: "white",
                  border: "none",
                  borderRadius: "12px",
                  cursor: loading ? "not-allowed" : "pointer",
                  transition: "all 0.3s ease",
                  boxShadow: loading
                    ? "none"
                    : "0 4px 15px rgba(16, 185, 129, 0.4)",
                }}
                onMouseOver={(e) => {
                  if (!loading)
                    e.currentTarget.style.backgroundColor = "#059669";
                }}
                onMouseOut={(e) => {
                  if (!loading)
                    e.currentTarget.style.backgroundColor = "#10b981";
                }}
              >
                {loading ? "⏳ Cargando..." : "💰 Ver mi Balance CODEBIT"}
              </button>
            </div>

            <div
              style={{
                padding: "30px",
                background: "linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%)",
                borderRadius: "12px",
                textAlign: "center",
                marginBottom: "20px",
              }}
            >
              <p
                style={{
                  fontSize: "14px",
                  margin: "0 0 8px 0",
                  color: "#065f46",
                  fontWeight: "600",
                }}
              >
                💎 Tu Balance Actual:
              </p>
              <div
                style={{
                  fontSize: "36px",
                  fontWeight: "bold",
                  margin: "15px 0",
                  color: "#065f46",
                }}
              >
                {Number(balance).toLocaleString("es-AR")} CODEBIT
              </div>
              <p
                style={{
                  fontSize: "13px",
                  margin: "8px 0 0 0",
                  color: "#047857",
                }}
              >
                (sin decimales - tokens enteros)
              </p>
            </div>

            <div
              style={{
                marginTop: "20px",
                padding: "15px",
                backgroundColor: "#fef3c7",
                borderRadius: "12px",
                border: "2px solid #fbbf24",
              }}
            >
              <p
                style={{
                  fontSize: "13px",
                  margin: 0,
                  color: "#92400e",
                  wordBreak: "break-all",
                }}
              >
                ℹ️ <strong>Contract ID:</strong>
                <br />
                <code style={{ fontSize: "11px" }}>{CONTRACT_ID}</code>
              </p>
            </div>

            <div
              style={{
                marginTop: "30px",
                padding: "25px",
                backgroundColor: "#f9fafb",
                borderRadius: "12px",
                border: "2px solid #e5e7eb",
              }}
            >
              <h3
                style={{
                  margin: "0 0 20px 0",
                  color: "#374151",
                  fontSize: "20px",
                }}
              >
                💸 Transferir Tokens
              </h3>

              <input
                type="text"
                placeholder="Dirección destino (G...)"
                value={recipient}
                onChange={(e) => setRecipient(e.target.value)}
                style={{
                  width: "100%",
                  padding: "12px",
                  marginBottom: "15px",
                  borderRadius: "8px",
                  border: "2px solid #d1d5db",
                  fontSize: "14px",
                  boxSizing: "border-box",
                }}
              />

              <input
                type="number"
                placeholder="Cantidad de CODEBIT (ej: 1000)"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                style={{
                  width: "100%",
                  padding: "12px",
                  marginBottom: "15px",
                  borderRadius: "8px",
                  border: "2px solid #d1d5db",
                  fontSize: "14px",
                  boxSizing: "border-box",
                }}
              />

              <button
                type="button"
                onClick={() => void handleTransfer()}
                disabled={loading}
                style={{
                  width: "100%",
                  padding: "12px",
                  background: loading
                    ? "#ccc"
                    : "linear-gradient(135deg, #2563eb 0%, #14b8a6 100%)",
                  color: "white",
                  border: "none",
                  borderRadius: "8px",
                  fontSize: "16px",
                  fontWeight: "bold",
                  cursor: loading ? "not-allowed" : "pointer",
                }}
              >
                {loading ? "Procesando..." : "Transferir"}
              </button>

              {message && (
                <p
                  style={{
                    marginTop: "15px",
                    padding: "10px",
                    backgroundColor: message.includes("✅")
                      ? "#d1fae5"
                      : "#fee2e2",
                    color: message.includes("✅") ? "#065f46" : "#991b1b",
                    borderRadius: "8px",
                    textAlign: "center",
                    fontSize: "14px",
                  }}
                >
                  {message}
                </p>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
