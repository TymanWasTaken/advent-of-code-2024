let
    # Import nixpkgs lib and input file
    inherit ((builtins.getFlake (builtins.toString ./..)).inputs.nixpkgs) lib;
    input = builtins.readFile (builtins.toString ./input.txt);

    # Define functions
    abs = n: if n < 0 then (-n) else n;

    # Do parsing
    parsed = ( # Define function to sort lines into two lists
        lib.lists.foldr
            (cur: acc: {
                first = acc.first ++ [(lib.strings.toIntBase10 (builtins.elemAt cur 0))];
                second = acc.second ++ [(lib.strings.toIntBase10 (builtins.elemAt cur 1))];
            })
            { first = []; second = []; }
    ) ( # Call with input split on newlines and 3 spaces; formatted like [ [ 1 2 ] [ 3 4 ] [ 5 6 ] ]
        lib.lists.forEach
            (lib.strings.splitString "\n" input)
            (e: lib.strings.splitString "   " e)
    );
    sorted = {
        first = lib.lists.sort (a: b: a < b) parsed.first;
        second = lib.lists.sort (a: b: a < b) parsed.second;
    };
    reorganized = (
        lib.lists.imap0
            (i: e: [ e (builtins.elemAt sorted.second i) ])
            sorted.first
    );

    output = (
        (
            lib.lists.foldr
                (cur: acc:
                    acc + (abs ((builtins.elemAt cur 0) - (builtins.elemAt cur 1)))
                )
                0
        ) reorganized
    );
in output