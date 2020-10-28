singleQ = StringQ[#] && !StringContainsQ[#, "."]&;
find[i_Integer] := Block[
	{url, ms, fs},
	url = <|
		"Scheme" -> "https",
		"Domain" -> "www.ebi.ac.uk",
		"Path" -> {"chembl", "api", "data", "molecule"},
		"Query" -> {
			"format" -> "json",
			"sort_by" -> "molecule_properties__mw_freebase",
			"molecule_properties__mw_freebase__range" -> StringRiffle[{i, i + 1}, ","]
		}
	|>;
	ms = Import[URLBuild@url, "RawJSON"]["molecules"];
	fs = Select[#["molecule_structures", "canonical_smiles"]& /@ ms, singleQ];
	If[ms == {}, i -> Missing[i], i -> RandomChoice@fs]
];

target = Normal@StringToByteArray["Hello world!", "UTF8"];
output = ParallelMap[find, target];
plot = MoleculePlot /@ ImportString[StringRiffle[Values@output, "\n"], "SMILES"];
Export["hello-world.svg", Multicolumn[plot, 3]]