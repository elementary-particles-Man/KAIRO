import unittest
import hashlib
import struct
from unittest.mock import patch, MagicMock

# Assuming generate_kairo_pcap.py is in the scripts directory
import sys
import os
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../scripts')))

from generate_kairo_pcap import _build_packet, main

class TestGenerateKairoPcap(unittest.TestCase):

    @patch('generate_kairo_pcap.uuid.uuid4')
    @patch('generate_kairo_pcap.hashlib.sha256')
    def test_build_packet(self, mock_sha256, mock_uuid4):
        # Mock UUID and SHA256 for deterministic testing
        mock_uuid_obj = MagicMock()
        mock_uuid_obj.bytes = b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01'
        mock_uuid4.return_value = mock_uuid_obj

        mock_sha256_digest = MagicMock()
        mock_sha256_digest.digest.side_effect = [b'\x00' * 32, b'\x01' * 32] # For sha and sig

        mock_sha256.return_value = mock_sha256_digest

        seq_id = 1
        packet = _build_packet(seq_id)

        # Basic checks on the packet structure
        self.assertIsInstance(packet, bytes)
        self.assertGreater(len(packet), 0)

        # Verify parts of the packet (e.g., IPv6 header, dest opts)
        # This is a simplified check, a more robust test would parse the packet
        # and verify specific fields.
        # For example, check the version and traffic class (first byte of IPv6 header)
        self.assertEqual((packet[0] >> 4), 6) # IPv6 version

        # Check payload length (bytes 4-5 of IPv6 header)
        payload_len = struct.unpack("!H", packet[4:6])[0]
        # The expected payload length is the length of dest_opts
        # option_data = uid.bytes + struct.pack("!I", seq_id) + sha + sig
        # len(option_data) = 16 (uuid) + 4 (seq_id) + 32 (sha) + 32 (sig) = 84
        # option_header = struct.pack("!BB", 0x63, len(option_data)) -> 2 bytes
        # dest_opts = struct.pack("!BB", 59, 10) + option_header + option_data
        # len(dest_opts) = 2 + 2 + 84 = 88
        self.assertEqual(payload_len, 88)

        # Check next header (byte 6 of IPv6 header) - should be 60 for Destination Options
        self.assertEqual(packet[6], 60)

        # Check hop limit (byte 7 of IPv6 header) - should be 64
        self.assertEqual(packet[7], 64)

        # Check source and destination IPs (bytes 8-23 and 24-39 of IPv6 header)
        self.assertEqual(packet[8:24], bytes.fromhex("20010db8000000000000000000000001"))
        self.assertEqual(packet[24:40], bytes.fromhex("20010db8000000000000000000000002"))

        # Check Destination Options header (first two bytes after IPv6 header)
        self.assertEqual(packet[40:42], struct.pack("!BB", 59, 10))

        # Check option header (bytes 42-44)
        self.assertEqual(packet[42:44], struct.pack("!BB", 0x63, 84))

        # Check option data (bytes 44 onwards)
        expected_option_data = mock_uuid_obj.bytes + struct.pack("!I", seq_id) + (b'\x00' * 32) + (b'\x01' * 32)
        self.assertEqual(packet[44:], expected_option_data)

    @patch('generate_kairo_pcap.Path')
    @patch('generate_kairo_pcap.os.environ')
    @patch('generate_kairo_pcap.time.time')
    @patch('generate_kairo_pcap._build_packet')
    def test_main_generates_pcap_file(self, mock_build_packet, mock_time, mock_environ, mock_path):
        mock_environ.get.return_value = None # Not read-only

        # Mock Path object and its methods
        mock_out_path = MagicMock()
        mock_path.return_value = mock_out_path
        mock_out_path.resolve.return_value = mock_out_path
        mock_out_path.parents.__getitem__.return_value = mock_out_path.parent
        mock_out_path.parent.mkdir.return_value = None

        # Mock file writing
        mock_open = MagicMock()
        mock_file_handle = MagicMock()
        mock_open.return_value.__enter__.return_value = mock_file_handle
        
        with patch('builtins.open', mock_open):
            mock_build_packet.return_value = b'\xDE\xAD\xBE\xEF' # Dummy packet
            mock_time.return_value = 1234567890 # Deterministic timestamp

            main()

            # Verify file was opened in binary write mode
            mock_open.assert_called_once()

            # Verify PCAP global header was written
            # struct.pack("<IHHIIII", 0xA1B2C3D4, 2, 4, 0, 0, 65535, 101)
            mock_file_handle.write.assert_any_call(struct.pack("<IHHIIII", 0xA1B2C3D4, 2, 4, 0, 0, 65535, 101))

            # Verify packets were written (3 packets in main)
            # Each packet writes a header and the packet data
            # struct.pack("<IIII", ts + i, 0, len(pkt), len(pkt))
            self.assertEqual(mock_file_handle.write.call_count, 1 + (3 * 2)) # 1 for global header + 3 * (packet header + packet data)

            # Verify packet data was written
            mock_file_handle.write.assert_any_call(b'\xDE\xAD\xBE\xEF')

if __name__ == '__main__':
    unittest.main()
