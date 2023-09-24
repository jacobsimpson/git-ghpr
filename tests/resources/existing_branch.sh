#! /bin/bash

mkdir tmp
cd tmp

git init .
git branchless init --main-branch main
echo "Some text" > README.md
git add README.md
git commit -m "Initial commit."
# Switch from the branch name to the hash as the currently selected pointer.
git checkout $(git rev-parse HEAD)
echo "More text" > file1.txt
git add file1.txt
git commit -m "Commit 2."
git checkout -b new-branch-name

tar -zcf ../existing_branch.tar.gz .
cd ..
rm -Rf tmp
