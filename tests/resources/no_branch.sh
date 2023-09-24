#! /bin/bash

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

    # Switch from the branch name to the hash as the currently selected pointer.
    git checkout $(git rev-parse HEAD)
    echo "More text" > file1.txt
    git add file1.txt
    git commit -m "Commit 2."
)

tar -zcf ../$(echo $(basename $0) | sed 's|\.sh||').tar.gz .
cd ..
rm -Rf tmp
