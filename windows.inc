; windows.inc
;
; Определения констант и структур Windows API для x64 NASM
;

; Константы для окна
WM_DESTROY      equ 2
WM_PAINT        equ 15

; Константы для класса окна
COLOR_WINDOW    equ 5
CS_HREDRAW      equ 2
CS_VREDRAW      equ 1

; Константы стилей окна
WS_OVERLAPPEDWINDOW equ 13565952
WS_VISIBLE          equ 134217728
WS_EX_APPWINDOW     equ 262144

; Константы для отображения окна
SW_SHOW         equ 5
