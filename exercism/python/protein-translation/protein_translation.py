def proteins(strand):
    translation = {
        "AUG": "Methionine",

        "UUU": "Phenylalanine",
        "UUC": "Phenylalanine",

        "UUA": "Leucine",
        "UUG": "Leucine",

        "UCU": "Serine",
        "UCC": "Serine",
        "UCA": "Serine",
        "UCG": "Serine",

        "UAU": "Tyrosine",
        "UAC": "Tyrosine",

        "UGU": "Cysteine",
        "UGC": "Cysteine",

        "UGG": "Tryptophan",

        "UAA": "STOP",
        "UAG": "STOP",
        "UGA": "STOP",
    }

    codons = [strand[i:i + 3] for i in range(0, len(strand), 3)]

    aminoacids = []
    for codon in codons:
        if translation[codon] is "STOP":
            break

        aminoacids.append(translation[codon])

    return aminoacids
