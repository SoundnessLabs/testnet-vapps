// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract MusicNFT is ERC721, Ownable {
    uint256 private _tokenIds;

    struct Track {
        string title;
        string uri; // link to audio file stored in Soundness Layer / IPFS / blob://...
    }

    mapping(uint256 => Track) public tracks;

    constructor() ERC721("MusicNFT", "MUSIC") {}

    function mintTrack(address recipient, string memory title, string memory uri)
        public
        onlyOwner
        returns (uint256)
    {
        _tokenIds += 1;
        uint256 newId = _tokenIds;

        _mint(recipient, newId);
        tracks[newId] = Track(title, uri);

        return newId;
    }

    function getTrack(uint256 tokenId) public view returns (string memory, string memory) {
        require(_exists(tokenId), "Track does not exist");
        Track memory track = tracks[tokenId];
        return (track.title, track.uri);
    }
}
