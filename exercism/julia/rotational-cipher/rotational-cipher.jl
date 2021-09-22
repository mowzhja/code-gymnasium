"""Rotates the character `plainchar` by an amount=`amount`."""
function rotate(amount, plainchar::Char)
    if !isletter(plainchar)
        return plainchar
    end

    shifted = plainchar + amount

    if isuppercase(plainchar)
        return getindex('A':'Z', (shifted - 'A') % 26 + 1)
    end

    return getindex('a':'z', (shifted - 'a') % 26 + 1)
end

"""Rotates the string `plaintext` by an amount=`amount`."""
rotate(amount, plaintext::String) = join(
  rotate(amount, character) for character in plaintext
  )
