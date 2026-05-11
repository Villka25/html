; main.asm
BITS 64
;
; Простая GUI-программа на ассемблере для Windows.
; Создает окно и выводит в нем текст "Hello from ASM!".
;
; Сборка и линковка (локально, без CI):
;   nasm -f win64 main.asm -o main.obj
;   golink main.obj /console /entry main user32.dll kernel32.dll gdi32.dll

; ----- Внешние функции из системных библиотек -----
extern GetModuleHandleW
extern GetStockObject
extern RegisterClassExW
extern CreateWindowExW
extern ShowWindow
extern UpdateWindow
extern GetMessageW
extern TranslateMessage
extern DispatchMessageW
extern DefWindowProcW
extern PostQuitMessage
extern BeginPaint
extern EndPaint
extern TextOutW
extern ExitProcess

; ----- Константы -----
%include "windows.inc"

; ----- Секция данных -----
section .data
    ; Имя класса окна
    class_name dw 'M','y','W','i','n','C','l','a','s','s',0
    ; Заголовок окна
    window_title dw 'A','S','M',' ','P','r','o','g','r','a','m',0
    ; Текст для вывода
    output_text  dw 'H','e','l','l','o',' ','f','r','o','m',' ','A','S','M','!',0

    ; Структура WNDCLASSEXW
    wc:
        .cbSize        dd 0          ; размер структуры
        .style         dd 0          ; стиль класса
        .lpfnWndProc   dq 0          ; указатель на оконную процедуру
        .cbClsExtra    dd 0
        .cbWndExtra    dd 0
        .hInstance     dq 0
        .hIcon         dq 0
        .hCursor       dq 0
        .hbrBackground dq 0
        .lpszMenuName  dq 0
        .lpszClassName dq 0
        .hIconSm       dq 0

    ; Структура для сообщения
    msg:
        .hwnd      dq 0
        .message   dd 0
        .wParam    dq 0
        .lParam    dq 0
        .time      dd 0
        .pt_x      dd 0
        .pt_y      dd 0

    ; Структура для Paint
    ps:
        .hdc         dq 0
        .fErase      db 0
        .rcPaint_left   dd 0
        .rcPaint_top    dd 0
        .rcPaint_right  dd 0
        .rcPaint_bottom dd 0
        .fRestore    db 0
        .fIncUpdate  db 0
        .rgbReserved db 32 dup (0)

; ----- Секция кода -----
section .text
    global main

main:
    ; Получаем хендл приложения
    sub     rsp, 40                 ; выравнивание стека
    xor     ecx, ecx                ; lpModuleName = NULL
    call    GetModuleHandleW
    mov     [wc.hInstance], rax

    ; Заполняем WNDCLASSEXW
    mov     dword [wc.cbSize], 80   ; sizeof(WNDCLASSEXW)
    mov     dword [wc.style], CS_HREDRAW | CS_VREDRAW
    mov     qword [wc.lpfnWndProc], WndProc
    mov     qword [wc.lpszClassName], class_name

    ; Кисть для фона (COLOR_WINDOW + 1)
    mov     ecx, COLOR_WINDOW
    add     ecx, 1
    call    GetStockObject
    mov     [wc.hbrBackground], rax

    ; Регистрируем класс окна
    sub     rsp, 32
    mov     ecx, wc
    call    RegisterClassExW
    add     rsp, 32

    ; Создаем окно
    ; CreateWindowExW(dwExStyle, lpClassName, lpWindowName, dwStyle, x, y, nWidth, nHeight, hWndParent, hMenu, hInstance, lpParam)
    sub     rsp, 64
    mov     ecx, WS_EX_APPWINDOW          ; dwExStyle
    mov     rdx, class_name               ; lpClassName
    mov     r8,  window_title             ; lpWindowName
    mov     r9d, WS_OVERLAPPEDWINDOW | WS_VISIBLE ; dwStyle
    mov     qword [rsp+32], CW_USEDEFAULT ; x
    mov     qword [rsp+40], CW_USEDEFAULT ; y
    mov     qword [rsp+48], CW_USEDEFAULT ; nWidth
    mov     qword [rsp+56], CW_USEDEFAULT ; nHeight
    mov     qword [rsp+64], 0             ; hWndParent
    mov     qword [rsp+72], 0             ; hMenu
    mov     rax, [wc.hInstance]
    mov     qword [rsp+80], rax           ; hInstance
    mov     qword [rsp+88], 0             ; lpParam
    call    CreateWindowExW
    add     rsp, 64

    ; Сохраняем HWND
    mov     rbx, rax

    ; Показываем окно
    sub     rsp, 40
    mov     ecx, ebx
    mov     rdx, SW_SHOW
    call    ShowWindow

    ; Обновляем окно
    mov     ecx, ebx
    call    UpdateWindow
    add     rsp, 40

.message_loop:
    ; GetMessage(&msg, NULL, 0, 0)
    sub     rsp, 40
    mov     ecx, msg
    xor     rdx, rdx
    xor     r8, r8
    xor     r9, r9
    call    GetMessageW
    add     rsp, 40

    cmp     eax, 0
    je      .exit_loop

    ; TranslateMessage(&msg)
    sub     rsp, 32
    mov     ecx, msg
    call    TranslateMessage
    add     rsp, 32

    ; DispatchMessage(&msg)
    sub     rsp, 32
    mov     ecx, msg
    call    DispatchMessageW
    add     rsp, 32

    jmp     .message_loop

.exit_loop:
    ; Выход
    xor     ecx, ecx
    call    ExitProcess

; ----- Оконная процедура -----
WndProc:
    ; Сохраняем регистры
    push    rbp
    mov     rbp, rsp
    sub     rsp, 64

    ; Сохраняем аргументы
    mov     r12, rcx        ; hWnd
    mov     r13d, edx       ; uMsg
    mov     r14, r8         ; wParam
    mov     r15, r9         ; lParam

    cmp     r13d, WM_DESTROY
    je      .destroy

    cmp     r13d, WM_PAINT
    je      .paint

    ; Вызываем стандартную обработку по умолчанию
    .default:
        mov     rcx, r12
        mov     rdx, r13
        mov     r8,  r14
        mov     r9,  r15
        call    DefWindowProcW
        jmp     .return

    .destroy:
        xor     ecx, ecx
        call    PostQuitMessage
        xor     eax, eax
        jmp     .return

    .paint:
        ; Начинаем рисование
        sub     rsp, 32
        mov     rcx, r12
        mov     rdx, ps
        call    BeginPaint

        ; Получаем DC (device context) из результата BeginPaint
        mov     rbx, rax

        ; Выводим текст
        ; Выравниваем стек перед вызовом
        sub     rsp, 64
        mov     rcx, rbx        ; hDC
        mov     rdx, 10         ; x
        mov     r8,  10         ; y
        mov     r9,  output_text ; lpString
        mov     qword [rsp+32], 16   ; cchString (длина строки)
        call    TextOutW
        add     rsp, 64

        ; Заканчиваем рисование
        mov     rcx, r12
        mov     rdx, ps
        call    EndPaint
        xor     eax, eax
        jmp     .return

.return:
    add     rsp, 64
    pop     rbp
    ret
