-- Run it with $ sqlite3 :memory: < part1.sql

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
    select l, li, ch from input_lines_raw where ch != 0 -- cleaning the line breaks
),
first_digits(l, li, ch) as (
    select
        l,
        li,
        max(ch) as ch
    from input_lines
    where li <= (select line_length from day3) - (select joltage_length from day3)
    group by l
-- ) select * from first_digits;/*,
),
unordered_digits as (
    select
        row_number() over (partition by input_lines.l order by input_lines.l, input_lines.ch desc, input_lines.li) as relative_rowid,
        row_number() over (order by input_lines.l, input_lines.ch desc, input_lines.li) as rowid,
        input_lines.l as input_line_l,
        input_lines.li as input_line_li,
        input_lines.ch as input_line_ch
    from input_lines
    order by input_lines.l, input_lines.ch desc, input_lines.li
)
-- ) select * from unordered_digits; /*
,
iterator(helper, i, l, li, ch, is_selected, n) as (
-- iterator(i, l, li, ch, is_selected, n) as (
    select
        false,
        0, -- first rowid
        0,
        0,
        '-',
        -- false, -- not necesarily the maximum
        false,
        0 -- start from 0, this is the first iteration
    from unordered_digits where rowid = 1 and input_line_l = 0
    union all
    select
        (
            select string_agg(d.input_line_ch, ', ')
            from unordered_digits d
            where
                d.input_line_l = unordered_digits.input_line_l
                and d.input_line_ch > unordered_digits.input_line_ch
                and d.input_line_li > unordered_digits.input_line_li
        ),
        iterator.i + 1 - (unordered_digits.input_line_l - iterator.l),
        unordered_digits.input_line_l,
        unordered_digits.input_line_li,
        unordered_digits.input_line_ch,
        -- case
        --     when unordered_digits.input_line_l != iterator.l then -1
        --     when (
        --         case
        --             when n = 0 and (unordered_digits.input_line_li >= ((select line_length from day3) - (select joltage_length from day3))) then true
        --             when ((select line_length from day3) - unordered_digits.relative_rowid + 1 = (select joltage_length from day3) - n) then true
        --             when n >= (select joltage_length from day3) then false --- maybe unneeded?
        --             -- when ((select line_length from day3) - unordered_digits.input_line_li >= (select joltage_length from day3) - n) then true
        --             when unordered_digits.input_line_li >= first_digits.li then true
        --             else false
        --         end
        --     ) and currmax == -1 and (unordered_digits.input_line_li < ((select line_length from day3) - (select joltage_length from day3))) then unordered_digits.input_line_li
        --     else currmax
        -- end,
        -- case
        --     when unordered_digits.input_line_l != iterator.l then -1
        --     -- if we don't find a first digit yet, then we accept anything that is within the allowed range
        --     when currmax = -1 and (unordered_digits.input_line_li >= ((select line_length from day3) - (select joltage_length from day3))) then currmax
        --     -- when we have the *same* amount of characters to try than we need to fill the joltage_length
        --     when ((select line_length from day3) - unordered_digits.relative_rowid + 1 = (select joltage_length from day3) - n) then unordered_digits.input_line_li
        --     -- when we go to a new line, we reset
        --     -- if we have enough characters to reach the joltage_length
        --     when n >= (select joltage_length from day3) then currmax
        --     -- if the current position doesn't fit for the amount of characters that are left, skip it
        --     --  when ((select line_length from day3) - unordered_digits.input_line_li >= (select joltage_length from day3) - n) then unordered_digits.input_line_li
        --     --
        --     -- when unordered_digits.input_line_li >= iterator.currmax then unordered_digits.input_line_li
        --     else currmax
        -- end,
        case
            when n = 0 and (unordered_digits.input_line_li >= ((select line_length from day3) - (select joltage_length from day3))) then true
            when ((select line_length from day3) - unordered_digits.relative_rowid + 1 = (select joltage_length from day3) - n) then true
            when n >= (select joltage_length from day3) then false --- maybe unneeded?
            -- when ((select line_length from day3) - unordered_digits.input_line_li >= (select joltage_length from day3) - n) then true
            when exists (
                select string_agg(d.input_line_ch, ', ')
                from unordered_digits d
                where d.input_line_l = unordered_digits.input_line_l and d.input_line_ch > unordered_digits.input_line_ch and d.input_line_li > unordered_digits.input_line_li
            ) then false
            when unordered_digits.input_line_li >= first_digits.li then true
            else false
        end,
        case
            when unordered_digits.input_line_l != iterator.l then 0
            when n = 0 and (unordered_digits.input_line_li >= ((select line_length from day3) - (select joltage_length from day3))) then n + 1
            when ((select line_length from day3) - unordered_digits.relative_rowid + 1 = (select joltage_length from day3) - n) then n + 1
            when n >= (select joltage_length from day3) then n
            -- when ((select line_length from day3) - unordered_digits.input_line_li >= (select joltage_length from day3) - n) then n + 1
            when exists (
                select string_agg(d.input_line_ch, ', ')
                from unordered_digits d
                where d.input_line_l = unordered_digits.input_line_l and d.input_line_ch > unordered_digits.input_line_ch and d.input_line_li > unordered_digits.input_line_li
            ) then n
            when unordered_digits.input_line_li >= first_digits.li then n + 1
            else n
        end
    from iterator
    join unordered_digits on unordered_digits.rowid = iterator.i + 1
    join first_digits on first_digits.l = iterator.l
) select * from iterator;/*

), ordered_digits as (
    select * from iterator where is_selected = true order by l, li
), answers as (
    select
        ordered_digits.l as l,
        cast(string_agg(ordered_digits.ch, '') as integer) as num
        from ordered_digits
        group by ordered_digits.l
) select * from answers;
-- ) select sum(num) from answers;


-- ,
-- -- unordered by ???? i don't even remember xdxdxdx
-- unordered_digits as (
--     select * from (
--         select
--             row_number() over (partition by input_lines.l order by input_lines.l, input_lines.ch desc, input_lines.li) as rowid,
--             input_lines.l as input_line_l,
--             input_lines.li as input_line_li,
--             input_lines.ch as input_line_ch,
--             first_digits.l as first_digits_l,
--             first_digits.li as first_digits_li,
--             first_digits.ch as first_digits_ch
--         from input_lines
--             join first_digits on input_lines.l = first_digits.l
--         where input_lines.li > first_digits.li
--         order by input_lines.l, input_lines.ch desc, input_lines.li
--     ) where rowid > (select line_length from day3) - first_digits_li - (select joltage_length from day3)
-- ) select * from unordered_digits;,

-- cutted_digits as (
--     select * from unordered_digits order by input_line_l, input_line_li
-- ),
-- answers as (
--     select
--         cast(first_digits.ch || string_agg(cutted_digits.input_line_ch, '') as integer) as val
--     from cutted_digits join first_digits on cutted_digits.input_line_l = first_digits.l
--     group by cutted_digits.input_line_l
-- ) select * from answers;
