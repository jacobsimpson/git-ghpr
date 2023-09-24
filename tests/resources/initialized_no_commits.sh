#! /bin/bash

cd $(dirname $0)
mkdir tmp
cd tmp

git init .
git branchless init --main-branch main
echo "Some text" > README.md

tar -zcf ../initialized_no_commits.tar.gz .
cd ..
rm -Rf tmp
