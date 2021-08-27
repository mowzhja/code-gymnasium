"""
  distance(a, b)

  Compute the Hamming distance between two strands of DNA, `a` and `b`.

  If the two strands have different length, the function will throw an error.
"""
function distance(a, b)
    if length(a) ≠ length(b)
        throw(ArgumentError("the two DNA strands need to have the same length"))
    # redundant but clear
    elseif isempty(a) && isempty(b) 
        return 0
    end

    sum(a[i] ≠ b[i] for i in 1:length(a))
end