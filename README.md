- Todo
  - Batched search
  - Cache on memory
    - The engine returns names of the files, not its content. That means I have to open the same file over and over per each search. Doesn't that seem inefficient?
  - Pipeline
    - If the search result is big, it can be pipelined.
    - The result is returned in stream, through pipe.
      - Let's use multiprocessing module from my game engine!
  - Incremental update
    - append/remove file incrementally
  - mutex
    - remove all mutex-related stuffs!