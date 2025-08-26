const fetch = require('node-fetch');

async function getGasPrice() {
  try {
    const response = await fetch('https://ethgasstation.info/api/ethgasAPI.json?api-key=YOUR_API_KEY');
    const data = await response.json();
    console.log("Gas Price (Gwei):", data.fast / 10);
  } catch (err) {
    console.error("Gagal mengambil data gas price:", err);
  }
}

getGasPrice();
