#! /bin/bash

cd $(dirname $0)
mkdir tmp
cd tmp

#
# Create the remote repository.
#
mkdir remote_repo
(
    cd remote_repo

    git init .
    git branchless init --main-branch main
    echo "Some text" > README.md
    git add README.md
    git commit -m "Initial commit."
)

#
# Clone the remote repository.
#
git clone remote_repo local_repo
(
    cd local_repo
    git branchless init

    git checkout -b base_branch
    echo "More text" > file1.txt
    git add file1.txt
    git commit -m "Commit 2."

    git checkout -b pr_branch
    echo "Even more text" > file2.txt
    git add file2.txt
    git commit -m "Commit 3."
)

tar -zcf ../$(echo $(basename $0) | sed 's|\.sh||').tar.gz .
cd ..
rm -Rf tmp
