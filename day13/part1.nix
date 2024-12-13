let
    # Import nixpkgs lib and input file
    inherit ((builtins.getFlake (builtins.toString ./..)).inputs.nixpkgs) lib;
    input = builtins.readFile (builtins.toString ./input.txt);

    # Define functions
    matElem = row: col: mat: builtins.elemAt (builtins.elemAt mat row) col;
    matDet = m: ((matElem 0 0 m) * (matElem 1 1 m)) - ((matElem 0 1 m) * (matElem 1 0 m));

    # Parse input
    split = builtins.map
        (s:
            let 
                split = lib.strings.splitString "\n" s;
            in  { a = builtins.elemAt split 0; b = builtins.elemAt split 1; prize = builtins.elemAt split 2; }
        )
        (lib.strings.splitString "\n\n" input);
    extracted = builtins.map
        ({ a, b, prize }: {
            a = lib.strings.splitString ", Y+" (builtins.substring 12 (-1) a);
            b = lib.strings.splitString ", Y+" (builtins.substring 12 (-1) b);
            prize = lib.strings.splitString ", Y=" (builtins.substring 9 (-1) prize);
        }) split;
    numbered = builtins.map
        ({ a, b, prize }: {
            a = { x = lib.strings.toInt (builtins.elemAt a 0); y = lib.strings.toInt (builtins.elemAt a 1); };
            b = { x = lib.strings.toInt (builtins.elemAt b 0); y = lib.strings.toInt (builtins.elemAt b 1); };
            prize = { x = lib.strings.toInt (builtins.elemAt prize 0); y = lib.strings.toInt (builtins.elemAt prize 1); };
        }) extracted;
    
    # Solve system of equations
    mathified = builtins.map
        ({ a, b, prize }: {
            # First equation
            a1 = a.x;
            b1 = b.x;
            c1 = prize.x;
            # Second equation
            a2 = a.y;
            b2 = b.y;
            c2 = prize.y;
        }) numbered;
    determinants = builtins.map
        ({ a1, b1, c1, a2, b2, c2 }: {
            numerators = {
                a = matDet [
                    [ c1 b1 ]
                    [ c2 b2 ]
                ];
                b = matDet [
                    [ a1 c1 ]
                    [ a2 c2 ]
                ];
            };
            denominator = matDet [
                [ a1 b1 ]
                [ a2 b2 ]
            ];
        }) mathified;
    tokens = (
        lib.lists.foldr
            ({ numerators, denominator }: accumulated-tokens:
                accumulated-tokens + (
                    if (
                        (lib.trivial.mod numerators.a denominator) != 0
                        || (lib.trivial.mod numerators.b denominator) != 0
                    )
                    then 0
                    else (numerators.a / denominator) * 3 + (numerators.b / denominator)
                )
            )
            0
    ) determinants;

    output = tokens;
in output