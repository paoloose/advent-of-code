-- Run it with
-- $ sqlite3 -box :memory: < part2.sql

with recursive
raw_input as (select readfile('input.txt')),
-- some variables to reuse
day3(txt, line_length, joltage_length, num_lines) as (
    select
        (select * from raw_input) as txt,
        instr((select * from raw_input), char(10)) - 1 as line_length,
        12 as joltage_length,
        length((select * from raw_input)) - length(replace((select * from raw_input), char(10), '')) as num_lines
),
input_lines_raw(l, gi, li, ch) as (
    select
        0, -- line index
        0, -- global char index
        0, -- local index (relative to line)
        cast(substr((select txt from day3), 1, 1) as integer) -- the current char
    union all
    select
        -- number of line breaks from the current line
        length(substr((select txt from day3), 0, gi + 2)) - length(replace(substr((select txt from day3), 0, gi + 2), char(10), '')),
        gi + 1,
        case when
            -- if the previous line is different from the current, we are in a new line, so start counting from zero
            l != (length(substr((select txt from day3), 0, gi + 2)) - length(replace(substr((select txt from day3), 0, gi + 2), char(10), '')))
                then 0
            else
                li + 1
        end,
        cast(substr((select txt from day3), gi + 2, 1) as integer)
    from input_lines_raw
    where l < (select num_lines from day3)
),
input_lines(l, li, ch) as (
    select
        l,
        li,
        ch
    from input_lines_raw where ch != 0 -- cleaning the line breaks
),
-- The problem mandates 12 characters answers
-- that is, this iterator need to run 12 times for each line of the input
-- however because of the fact that recursive CTEs need a dummy base row, we will iterate 13 times for each line
-- the first row will always be a dummy one, with pos = -1
-- the only purpose of that row is to reset the variables to initial states and prepare for the next input line
-- we detect a new line when
--   (select joltage_length from day3) - mod(i, (select joltage_length from day3) + 1) = (select joltage_length from day3),
-- an expression that you'll se repeated multiple times in the following query,
-- i would love to bind that value to a name but it seems not possible in sqlite
iterator(i, pos, remaining, l, ch) as (
    select
        -- initial dummy values
        1,
        -1, -- we keep track of the last selected character local index (li)
        (select joltage_length from day3)+1 as remaining,
        0,
        ''
    union all
    select
        -- the loop iteration
        i + 1,
        case
            -- new lines get its position resetted
            when (select joltage_length from day3) - mod(i, (select joltage_length from day3) + 1) = (select joltage_length from day3)
                then -1
            else (
            -- The main algorithm. It searchs in the input characters for the MAX(value) that is
            --   (* ) after out current pos
            --   (**) but before we ran out of space to not be able to select enough characters to reach 12 (joltage_length)
            -- This way we ensure we get the maxmimum next value from the tightest range possible
            -- The same expression is repeated later, this is for getting its index, and the later for getting its value
            select
                li
            from input_lines
            where
                input_lines.l = iterator.l and -- (* )
                input_lines.li > iterator.pos and -- (**)
                input_lines.li <= ((select line_length from day3) - iterator.remaining+1)
            order by ch desc
            limit 1
        ) end,
        -- remaining
        (select joltage_length from day3) - mod(i, (select joltage_length from day3) + 1) + 1,
        -- current line
        case
            when (select joltage_length from day3) - mod(i, (select joltage_length from day3) + 1) = (select joltage_length from day3)
                then iterator.l + 1
            else iterator.l
        end,
        (
            select
                max(ch)
            from input_lines
            where
                input_lines.l = iterator.l and
                input_lines.li > iterator.pos and
                input_lines.li <= ((select line_length from day3) - iterator.remaining+1)
        )
        from iterator
        limit (
            select (count(69) / (select line_length from day3)) * ((select joltage_length from day3) + 1) from input_lines
        )
),
answers as (
    select
        l,
        string_agg(iterator.ch, '') val
    from iterator
    where pos >= 0
    group by iterator.l
) select sum(val) from answers;
