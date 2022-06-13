

def ChangeText(s):
    print('ChangeText() received:', s)
    decoded = s.decode("utf-16-le")
    print('Decoded:', decoded)
    
    returned = None
    if decoded == 'hello':
        returned = 'world'
    
    if returned is None:
        return None
    else:
        b = returned.encode("utf-16-le") + b'\0\0'
        return b


if __name__ == '__main__':
    print(ChangeText(b'h\0e\0l\0l\0o\0'))
    input()
