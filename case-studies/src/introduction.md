# Introduction

Specifying and verifying properties about software is hard:
it is hard to know what is useful to check;
it is hard to know the best way to say what you want;
and it is hard to know when you have said enough.
This series of case studies uses small examples to explore
the different choices and trade-offs that we face when
specifying and verifying software.


We shall explore to ways of verifying our examples: "dynamically" by testing the
code and "statically" by formally verifying the code.
For  "dynamic verification,"
we shall use the [proptest](https://github.com/AltSysrq/proptest)
library to create property-based specifications for problems
and to test whether the solutions satisfy those specifications.
For "static verification,"
we shall use the [Rust Verification
Tools](https://github.com/project-oak/rust-verification-tools)
to formally verify whether the solutions satisfy the specifications.
As we use these two different approaches,
we are interested in seeing where both approaches seem equally
effective, where one approach seems to be better and we shall be alert for
examples where both approaches seem to be unsatisfactory.


Our focus is on specifications that lend themselves to verification by automated
tools such as symbolic execution, bounded model checking, model checkin, etc.
because we believe that these techniques have reached the stage where they can
help typical developers to achieve their goals more effectively.
More manual specification and verification approaches have their place but
they require significant training to become proficient and they require larger
changes in how software is developed.
