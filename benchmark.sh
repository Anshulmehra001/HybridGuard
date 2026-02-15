#!/bin/bash
# HybridGuard Performance Benchmark Script

echo "🚀 HybridGuard Performance Benchmark"
echo "======================================"
echo ""

# Build release version
echo "📦 Building release version..."
cargo build --release
echo ""

# Test different file sizes
SIZES=(1024 10240 102400 1048576)  # 1KB, 10KB, 100KB, 1MB
NAMES=("1KB" "10KB" "100KB" "1MB")

for i in "${!SIZES[@]}"; do
    SIZE=${SIZES[$i]}
    NAME=${NAMES[$i]}
    
    echo "📊 Testing $NAME file..."
    
    # Create test file
    dd if=/dev/urandom of=test_$NAME.bin bs=$SIZE count=1 2>/dev/null
    
    # Measure encryption time
    START=$(date +%s%N)
    ./target/release/hybridguard encrypt -i test_$NAME.bin -o test_$NAME.enc 2>/dev/null
    END=$(date +%s%N)
    ENCRYPT_TIME=$(( ($END - $START) / 1000000 ))  # Convert to milliseconds
    
    # Measure decryption time
    START=$(date +%s%N)
    ./target/release/hybridguard decrypt -i test_$NAME.enc -o test_$NAME.dec 2>/dev/null
    END=$(date +%s%N)
    DECRYPT_TIME=$(( ($END - $START) / 1000000 ))
    
    # Get file sizes
    ORIGINAL_SIZE=$(stat -f%z test_$NAME.bin 2>/dev/null || stat -c%s test_$NAME.bin)
    ENCRYPTED_SIZE=$(stat -f%z test_$NAME.enc 2>/dev/null || stat -c%s test_$NAME.enc)
    EXPANSION=$(echo "scale=2; $ENCRYPTED_SIZE / $ORIGINAL_SIZE" | bc)
    
    # Calculate throughput
    THROUGHPUT=$(echo "scale=2; $SIZE / $ENCRYPT_TIME * 1000 / 1024" | bc)
    
    echo "  ✅ Encryption: ${ENCRYPT_TIME}ms"
    echo "  ✅ Decryption: ${DECRYPT_TIME}ms"
    echo "  ✅ Expansion: ${EXPANSION}x"
    echo "  ✅ Throughput: ${THROUGHPUT} KB/s"
    echo ""
    
    # Cleanup
    rm -f test_$NAME.bin test_$NAME.enc test_$NAME.dec
done

echo "✅ Benchmark complete!"
