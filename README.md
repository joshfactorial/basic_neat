This is a proof of concept of building NEAT (https://github.com/ncsa/neat) in Rust. I think the way NEAT works would lend itself well to a compiled programming language, 
because most of the data structures were too big to make effective use of Python's many packages designed to process data (pandas, numpy, biopython). 

This contains a self-contained process to read in the E. coli baciterial genome and produce a fastq. I kept everything as simple as possible. To run it on your own
machine, you'll need to clone the repo and update the paths in different files. I believe the fastq_writer and the main files are the ones to update.

We'll be using this basic idea in the NEAT repo for a rust version of NEAT coming soon!
