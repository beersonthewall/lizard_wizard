from bs4 import BeautifulSoup

inputPath = "./opcodes.html"

def main():
    with open(inputPath, "r") as f:
        html = f.read()
        soup = BeautifulSoup(html, 'html.parser')
        rows = soup.find('table').find_all('tr')

        # Output constant decl
        print("pub const OPCODES: [[I; 16]; 16] = [")

        # skip 0th indexes (e.g. 0x0, 0x1, etc)
        for rowNum in range(1,17):
            rowOutput =["["]
            row = rows[rowNum]

            # skip 0th cell which is the indexes
            for cell in row.find_all('td')[1:]:
                data = cell.text.split()
                data = ''.join(data)
                i = 0
                while i < len(data) and data[i].isupper():
                    i += 1

                opcode = data[0:i]

                start = i
                if i >= len(data):
                    # no addr mode or cycles listed
                    rowOutput.append(f"I::new(Op::{opcode},0,AM::IMP),")
                    continue
                elif data[i].isdigit():
                    # No addressing mode listed, but we do have a cycle count
                    # default to implied addressing mode
                    cycles = int(data[i])
                    rowOutput.append(f"I::new(Op::{opcode},{cycles},AM::IMP),")
                    continue

                start = i
                while i < len(data) and data[i].islower():
                    i += 1
                addr_mode = data[start:i]

                # convert opcode to rust enum
                if addr_mode == 'abs':
                    addr_mode = 'AM::ABS'
                elif addr_mode == 'abx':
                    addr_mode = 'AM::ABX'
                elif addr_mode == 'aby':
                    addr_mode = 'AM::ABY'
                elif addr_mode == 'imm':
                    addr_mode = 'AM::IMM'
                elif addr_mode == 'ind':
                    addr_mode = 'AM::IND'
                elif addr_mode == 'izx':
                    addr_mode = 'AM::INX'
                elif addr_mode == 'izy':
                    addr_mode = 'AM::INY'
                elif addr_mode == 'zp':
                    addr_mode = 'AM::ZPG'
                elif addr_mode == 'zpx':
                    addr_mode = 'AM::ZPX'
                elif addr_mode == 'zpy':
                    addr_mode = 'AM::ZPY'
                elif addr_mode == 'rel':
                    addr_mode = 'AM::REL'
                elif addr_mode == '':
                    addr_mode = 'AM::IMP'
                cycles = int(data[i]) if i < len(data) else 0
                rowOutput.append(f"I::new(Op::{opcode},{cycles},{addr_mode}),")

            # Close the array
            rowOutput.append("]")
            print(''.join(rowOutput) + ',')

        # close the decl
        print("]")



if __name__ == '__main__':
    main()
