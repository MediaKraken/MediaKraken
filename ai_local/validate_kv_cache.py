#!/usr/bin/env python3
"""
Script to validate remote KV-cache connectivity before launch.
Designed primarily for Redis, but can be adapted for other KV stores.
"""

import sys
import socket
import time
from typing import Dict, Any, Optional, Tuple
from dataclasses import dataclass, field
from contextlib import contextmanager
import asyncio


@dataclass
class ValidationResult:
    """Result of a validation check"""
    status: str  # "PASS", "FAIL", "WARN"
    message: str
    details: Optional[Dict[str, Any]] = None


@contextmanager
def timing():
    """Context manager for timing operations"""
    start = time.time()
    yield
    end = time.time()
    print(f"  Operation completed in {(end - start)*1000:.2f}ms")


class KVCacheValidator:
    """Validates remote KV-cache connectivity"""
    
    def __init__(self, host: str = "localhost", port: int = 6379, 
                 protocol: str = "redis", timeout: float = 5.0):
        self.host = host
        self.port = port
        self.protocol = protocol
        self.timeout = timeout
        self.results: list[ValidationResult] = []
        
    def check_tcp_connection(self) -> ValidationResult:
        """Check if we can establish a TCP connection to the KV cache"""
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(self.timeout)
            
            with timing():
                sock.connect((self.host, self.port))
                
            sock.close()
            return ValidationResult(
                status="PASS",
                message=f"TCP connection to {self.host}:{self.port} successful",
                details={"host": self.host, "port": self.port}
            )
        except socket.timeout:
            return ValidationResult(
                status="FAIL",
                message=f"TCP connection timeout to {self.host}:{self.port}",
                details={"host": self.host, "port": self.port, "timeout": self.timeout}
            )
        except Exception as e:
            return ValidationResult(
                status="FAIL",
                message=f"TCP connection failed to {self.host}:{self.port}: {str(e)}",
                details={"host": self.host, "port": self.port, "error": str(e)}
            )
    
    def check_protocol(self) -> ValidationResult:
        """Check if the specified protocol is supported"""
        supported_protocols = ["redis", "memcached", " Aerospike"]
        
        if self.protocol in supported_protocols:
            return ValidationResult(
                status="PASS",
                message=f"Protocol '{self.protocol}' is supported",
                details={"protocol": self.protocol}
            )
        else:
            return ValidationResult(
                status="FAIL",
                message=f"Protocol '{self.protocol}' is not supported",
                details={"supported_protocols": supported_protocols}
            )
    
    def check_server_info(self) -> ValidationResult:
        """Check basic server info (can be extended with protocol-specific checks)"""
        # For Redis, we'd use redis-py library
        # For production, you'd want to use the actual client library
        return ValidationResult(
            status="WARN",
            message="Protocol-specific info check skipped - client library not available",
            details={"protocol": self.protocol}
        )
    
    def validate(self) -> Tuple[bool, list[ValidationResult]]:
        """Run all validation checks"""
        checks = [
            self.check_protocol,
            self.check_tcp_connection,
            self.check_server_info
        ]
        
        self.results = [check() for check in checks]
        
        # Check if all critical checks passed
        critical_passed = all(
            result.status in ("PASS", "WARN") 
            for result in self.results 
            if result.status != "WARN"
        )
        
        return critical_passed, self.results


def print_results(results: list[ValidationResult], summary_only: bool = False):
    """Print validation results in a readable format"""
    print("\n" + "=" * 60)
    print("KV-Cache Validation Results")
    print("=" * 60)
    
    for result in results:
        status_icon = {
            "PASS": "✓",
            "FAIL": "✗",
            "WARN": "⚠"
        }.get(result.status, "?")
        
        print(f"{status_icon} [{result.status}] {result.message}")
        
        if result.details and not summary_only:
            print(f"    Details: {result.details}")
    
    print("=" * 60)


def main():
    """Main function"""
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Validate remote KV-cache connectivity before launch"
    )
    parser.add_argument(
        "--host", "-H", default="localhost",
        help="KV-cache server host (default: localhost)"
    )
    parser.add_argument(
        "--port", "-p", type=int, default=6379,
        help="KV-cache server port (default: 6379 for Redis)"
    )
    parser.add_argument(
        "--protocol", default="redis",
        help="KV-cache protocol (default: redis)"
    )
    parser.add_argument(
        "--timeout", "-t", type=float, default=5.0,
        help="Connection timeout in seconds (default: 5.0)"
    )
    parser.add_argument(
        "--summary", "-s", action="store_true",
        help="Show only summary, not detailed results"
    )
    
    args = parser.parse_args()
    
    validator = KVCacheValidator(
        host=args.host,
        port=args.port,
        protocol=args.protocol,
        timeout=args.timeout
    )
    
    success, results = validator.validate()
    print_results(results, summary_only=args.summary)
    
    # Exit with appropriate code
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()