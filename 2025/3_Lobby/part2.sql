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
        length(substr((select txt from day3), 0, gi + 2)) - length(replace(substr((select txt from day3), 0, gi + 2), char(10), '')),
        gi + 1,
        case when
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
first_digits as (
    select
        l,
        li,
        max(ch) as ch
    from input_line
    where li <= (select line_length from day3) - 12
    group by l
),
second_digits(line) as (
    select
        0
    union all
    select
        x + 1
    from second_digits where x < (select line_length from day3)
) select * from second_digits;
select
    sum(d1.ch * 10 + d2.ch)
    from first_digits d1
    join second_digits d2
    on d1.l = d2.l;
