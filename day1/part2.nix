let
    # Import nixpkgs lib and input file
    inherit ((builtins.getFlake (builtins.toString ./..)).inputs.nixpkgs) lib;
    input = builtins.readFile (builtins.toString ./input.txt);

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
    collapsed = (
        lib.lists.foldr
            (cur: acc:
                acc // (
                    if (builtins.hasAttr "${builtins.toString cur}" acc)
                    then {
                        "${builtins.toString cur}" = acc."${builtins.toString cur}" + 1;
                    }
                    else {
                        "${builtins.toString cur}" = 1;
                    }
                )
            )
            {}
    ) parsed.second;

    output = (
        lib.lists.foldr
            (cur: acc:
                acc + (
                    if (builtins.hasAttr "${builtins.toString cur}" collapsed)
                    then (cur * collapsed."${builtins.toString cur}")
                    else 0
                )
            )
            0
    ) parsed.first;
in output