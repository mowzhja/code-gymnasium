"""
    is_leap_year(year)

Return `true` if `year` is a leap year in the gregorian calendar.

"""
function is_leap_year(year)
    # https://en.wikipedia.org/wiki/Leap_years#Algorithm
    if year % 4 != 0 
        return false
    elseif year % 100 != 0
        return true
    elseif year % 400 != 0
        return false
    end

    return true
end

    