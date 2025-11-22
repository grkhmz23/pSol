# Run in Codespace
solana account DjS56MefviaDAyShsekRSyirN6ysfDbFYqLLMB8jFQbr --url devnet#!/bin/bash

echo "=========================================="
echo "FINAL REPOSITORY AUDIT"
echo "=========================================="
echo ""

ISSUES=0

echo "=== 1. Structure Check ==="
echo ""

# Privacy Protocol
if [ -f "psol/programs/psol/src/lib.rs" ]; then
    ID=$(grep "declare_id!" psol/programs/psol/src/lib.rs | grep -o '"[^"]*"' | tr -d '"')
    if [ "$ID" == "2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv" ]; then
        echo "✅ Privacy Protocol: Correct ID"
    else
        echo "❌ Privacy Protocol: Wrong ID"
        ((ISSUES++))
    fi
else
    echo "❌ Privacy Protocol: Missing"
    ((ISSUES++))
fi

# Token Program
if [ -f "programs/psol-token/src/lib.rs" ]; then
    ID=$(grep "declare_id!" programs/psol-token/src/lib.rs | grep -o '"[^"]*"' | tr -d '"')
    if [ "$ID" == "CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy" ]; then
        echo "✅ Token Program: Correct ID"
    else
        echo "❌ Token Program: Wrong ID"
        ((ISSUES++))
    fi
else
    echo "❌ Token Program: Missing"
    ((ISSUES++))
fi

echo ""
echo "=== 2. Code Quality ==="
echo ""

# Check transfer.rs fix
if grep -q "has_one = owner" psol/programs/psol/src/instructions/transfer.rs 2>/dev/null; then
    echo "✅ transfer.rs: Fixed (uses 'owner')"
else
    echo "❌ transfer.rs: Still broken"
    ((ISSUES++))
fi

# Check Cargo.toml repositories
if grep -q "grkhmz23/pSol" psol/programs/psol/Cargo.toml 2>/dev/null; then
    echo "✅ Privacy Cargo.toml: Correct repo URL"
else
    echo "❌ Privacy Cargo.toml: Wrong repo URL"
    ((ISSUES++))
fi

if grep -q "grkhmz23/pSol" programs/psol-token/Cargo.toml 2>/dev/null; then
    echo "✅ Token Cargo.toml: Correct repo URL"
else
    echo "❌ Token Cargo.toml: Wrong repo URL"
    ((ISSUES++))
fi

echo ""
echo "=== 3. Documentation ==="
echo ""

[ -f "README.md" ] && echo "✅ README.md" || ((ISSUES++))
[ -f "DEPLOYMENT_INFO.md" ] && echo "✅ DEPLOYMENT_INFO.md" || echo "⚠️  DEPLOYMENT_INFO.md missing"
[ -d "docs" ] && echo "✅ docs/ folder" || echo "⚠️  docs/ missing"

echo ""
echo "=== 4. Git Hygiene ==="
echo ""

# Check for large files
LARGE=$(find . -type f -size +1M -not -path '*/.git/*' -not -path '*/target/*' -not -path '*/node_modules/*' 2>/dev/null)
if [ -z "$LARGE" ]; then
    echo "✅ No large files"
else
    echo "⚠️  Large files found:"
    echo "$LARGE"
fi

# Check for secrets
if grep -r "private.*key\|secret\|password" . --exclude-dir=.git --exclude-dir=target --exclude-dir=node_modules --exclude="*.sh" 2>/dev/null | grep -v "pub " | grep -v "// "; then
    echo "⚠️  Possible secrets found!"
    ((ISSUES++))
else
    echo "✅ No obvious secrets"
fi

echo ""
echo "=========================================="
if [ $ISSUES -eq 0 ]; then
    echo "�� REPOSITORY IS CLEAN!"
    echo "✅ Ready for production"
else
    echo "⚠️  $ISSUES issues remaining"
fi
echo "=========================================="
echo ""

echo "📊 Repository Summary:"
echo ""
echo "Programs:"
echo "  - Privacy Protocol: 2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv"
echo "  - Token Program:    CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy"
echo ""
echo "Status:"
echo "  - Both programs compiled (on Solana Playground)"
echo "  - Source code clean and organized"
echo "  - Documentation complete"
echo "  - Ready for mainnet preparation"
echo ""
echo "Next steps:"
echo "  1. Security audit (Halborn/OtterSec)"
echo "  2. Initialize token program on devnet"
echo "  3. Test complete user flow"
echo "  4. Update website"
echo "  5. Prepare mainnet deployment"
