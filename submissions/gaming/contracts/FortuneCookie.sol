// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract FortuneCookie {
    string[] public fortunes;
    mapping(address => string) public lastFortune;

    event CookieOpened(address indexed user, string fortune, uint256 index);

    constructor() {
        fortunes.push("Bull market is near 🚀");
        fortunes.push("Patience is the best trading strategy ⏳");
        fortunes.push("Don’t ape without research 🐒");
        fortunes.push("Your wallet will moon soon 🌕");
        fortunes.push("Diamond hands never fail 💎🙌");
        fortunes.push("NGOBI: No Greed, Only Balanced Investing 😌");
        fortunes.push("Cut losses fast, ride winners 🏇");
        fortunes.push("Bear teaches discipline 🐻");
        fortunes.push("DYOR saves your portfolio 🧭");
        fortunes.push("Fortune favors the calm 😎");
    }

    function openCookie() external returns (string memory) {
        require(fortunes.length > 0, "No fortunes");
        uint256 index = uint256(
            keccak256(
                abi.encodePacked(block.timestamp, msg.sender, block.prevrandao)
            )
        ) % fortunes.length;

        string memory fortune = fortunes[index];
        lastFortune[msg.sender] = fortune;
        emit CookieOpened(msg.sender, fortune, index);
        return fortune;
    }

    function getLastFortune(address user) external view returns (string memory) {
        return lastFortune[user];
    }

    function getFortunesCount() external view returns (uint256) {
        return fortunes.length;
    }

    // Optional admin add fortunes (remove if not needed)
    function _addFortune(string memory text) internal {
        fortunes.push(text);
    }
}
