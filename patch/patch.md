# 使用 patch 同步DEF变更

```bash
# 从两个提交间创建补丁文件
git diff commit_id1 commit_id2 > commmit_message.patch
# 检查补丁文件
git apply --check -v commmit_message.patch
# 应用补丁文件
git apply -v commmit_message.patch
```