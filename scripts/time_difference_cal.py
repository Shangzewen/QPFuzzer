# import time

def get_time_diff(time1, time2):
    time1_split_pre = time1.split(" ")
    time1_split_after = time1_split_pre[1].split(":")
    time2_split_pre = time2.split(" ")
    time2_split_after = time2_split_pre[1].split(":")
    time_diff = (int(time1_split_after[0]) - int(time2_split_after[0]))*3600 + (int(time1_split_after[1]) - int(time2_split_after[1]))*60 + (int(time1_split_after[2])-int(time2_split_after[2]))
    return str(time_diff)


if __name__ == '__main__':
    td = get_time_diff("15:14:30","13:50:14")
    print(td)