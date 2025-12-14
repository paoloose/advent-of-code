-- Run it with $ sqlite3 :memory: < part1.sql

with recursive
raw_input as (select readfile('input.txt')),
-- some variables to reuse
day3(txt, line_length, num_lines) as (
    select
        (select * from raw_input) as txt,
        instr((select * from raw_input), char(10)) - 1 as line_length,
        length((select * from raw_input)) - length(replace((select * from raw_input), char(10), '')) as num_lines
),
input_line_raw(l, gi, li, ch) as (
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
    from input_line_raw
    where l < (select num_lines from day3)
),
input_line(l, li, ch) as (
    select l, li, ch from input_line_raw where ch != 0 -- cleaning the line breaks
),
-- all first digits
first_digits(l, li, ch) as (
    select
        l,
        li,
        max(ch) as ch
    from input_line
    where li < (select line_length from day3) - 1
    group by l
),
-- all second digits
second_digits as (
    select
        l,
        li,
        max(ch) as ch
    from input_line
    where
        li > (select li from first_digits where first_digits.l = input_line.l limit 1) -- after the first
    group by l
)
-- merging and adding
select
    sum(d1.ch * 10 + d2.ch)
    from first_digits d1
    join second_digits d2
    on d1.l = d2.l;
