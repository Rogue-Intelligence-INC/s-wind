#!/bin/bash
# 测试更长的序列（更多数据点）是否能通过验证

echo "测试1: 更长的等差序列（7个点）"
/workspace/target/release/swind explore "52,45,38,31,24,17,10" --mode verified --save-pending --kb test_kb1.json

echo -e "\n测试2: 精确的线性序列（5个点）"
/workspace/target/release/swind explore "100,90,80,70,60" --mode verified --save-pending --kb test_kb2.json

echo -e "\n测试3: 斐波那契序列（已知序列）"
/workspace/target/release/swind explore "1,1,2,3,5,8,13" --mode verified --save-pending --kb test_kb3.json

echo -e "\n检查知识库文件"
ls -lh test_kb*.json 2>/dev/null || echo "没有知识库文件生成"
