"""
Advent of Code, Day 5
"""

def calculate_seat_id(seat_location_string: str) -> int:
    """Calc seat id."""
    row_string = seat_location_string[0:7]
    col_string = seat_location_string[7:]

    row_binary = row_string.replace('F', '0').replace('B', '1')
    col_binary = col_string.replace('R', '1').replace('L', '0')

    row_num = int(row_binary, 2)
    col_num = int(col_binary, 2)

    seat_id = row_num * 8 + col_num
    # print(f'seat_id:{seat_id} row_num:{row_num} col_num:{col_num}')

    return seat_id


def find_my_seat_id(seats: list) -> int:
    """Return ID of my seat."""
    old_seat_id = -1
    for seat_id in seats:
        if seat_id - old_seat_id == 2:
            return seat_id


if __name__ == '__main__':
    seat_ids = []

    with open('seat_codes.txt') as seats_fh:
        for seat in seats_fh.readlines():
            seat_ids.append(calculate_seat_id(seat.strip()))
    print(seat_ids)

    print(f'My seat ID: {find_my_seat_id(seat_ids)}')
