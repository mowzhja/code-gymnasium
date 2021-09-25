def saddle_points(matrix):
    if not all(len(row) == len(matrix[0]) for row in matrix):
        raise ValueError("the matrix is irregular")

    results = set()
    for r, row in enumerate(matrix):

        row_maxima = [(i, m) for i, m in enumerate(row) if m == max(row)]
        for c, row_maximum in row_maxima:
            column = [row[c] for row in matrix]

            results |= {(r, c)} if row_maximum is min(column) else set()

    return [{"row": r + 1, "column": c + 1} for r, c in results]
