import unittest
import struct
import hashlib
import uuid
from scripts.generate_kairo_pcap import _build_packet

class TestGenerateKairoPcap(unittest.TestCase):

    def test_build_packet_structure(self):
        # Test if _build_packet returns bytes
        packet = _build_packet(1)
        self.assertIsInstance(packet, bytes)

        # Test the length of the generated packet (this might need adjustment if the packet structure changes)
        # A basic check for a reasonable length, not an exact one due to UUID and SHA
        self.assertGreater(len(packet), 100)

        # Verify the IPv6 header version and traffic class (first byte)
        # (6 << 28) in _build_packet means the first byte should be 0x60
        self.assertEqual(packet[0] & 0xF0, 0x60)

        # Verify next header (byte 6) is 60 (Destination Options)
        self.assertEqual(packet[6], 60)

        # Verify hop limit (byte 7) is 64
        self.assertEqual(packet[7], 64)

    def test_build_packet_unique_elements(self):
        # Test if UUID and SHA are unique for different sequence IDs
        packet1 = _build_packet(1)
        packet2 = _build_packet(2)

        # Extract UUID from option_data (offset 40 from start of packet, 8 bytes for option_header, 16 bytes for UUID)
        # This is a bit fragile, but attempts to verify uniqueness
        # IPv6 header (40 bytes) + dest_opts header (2 bytes) + option_header (2 bytes) = 44 bytes
        # UUID is 16 bytes
        uuid1_bytes = packet1[44:60]
        uuid2_bytes = packet2[44:60]
        self.assertNotEqual(uuid1_bytes, uuid2_bytes)

        # Extract SHA from option_data (offset 60 from start of packet, 32 bytes for SHA)
        sha1_bytes = packet1[60:92]
        sha2_bytes = packet2[60:92]
        self.assertNotEqual(sha1_bytes, sha2_bytes)

    def test_build_packet_sequence_id(self):
        # Test if the sequence ID is correctly embedded
        seq_id = 123
        packet = _build_packet(seq_id)

        # The sequence ID is packed as !I (unsigned int) after the UUID (16 bytes) in option_data
        # IPv6 header (40 bytes) + dest_opts header (2 bytes) + option_header (2 bytes) + UUID (16 bytes) = 60 bytes
        # Sequence ID is at offset 60 and is 4 bytes long
        extracted_seq_id = struct.unpack("!I", packet[60:64])[0]
        self.assertEqual(extracted_seq_id, seq_id)

if __name__ == '__main__':
    unittest.main()