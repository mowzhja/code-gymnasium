"""Rotates the character `plainchar` by an amount=`n`."""
function rotate(n, plainchar::Char)
    if !isletter(plainchar)
        return plainchar
    end

    if isuppercase(plainchar)
        return getindex('A':'Z', (plainchar + n - 'A') % 26 + 1)
    end

    return getindex('a':'z', (plainchar + n - 'a') % 26 + 1)
end

"""Rotates the string `plaintext` by an amount=`n`."""
rotate(n, plaintext::String) = map(char -> rotate(n, char), plaintext) 
