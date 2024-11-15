org 100h
mov bx, 10h
mov ax, bx
mov cx, 0x1234
add cx, 0dcbah
mov [0h], cx
mov ax, [0h]
mov word ptr [1000h], 0x1234
mov cx, 0x1234
add word ptr [1000h], cx
add cx, word ptr [1000h]
;sub si, di
;mul cx
;div bx
;cmp ax, cx
;jmp 1
;1:
;mov ax, ax