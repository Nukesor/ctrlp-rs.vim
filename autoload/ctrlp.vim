let s:bin = resolve(expand('<sfile>:p:h') . '/../target/debug/ctrlp-rs')

" Default job id of 0 (no running job)
if ! exists('s:job_id')
    let s:job_id = 0
endif

function! ctrlp#init()
    let result = s:startup()

    if 0 == result
        echoerr "ctrlp-rs: Failed to start process"
    elseif -1 == result
        echoerr "ctrlp-rs: Binary not executable"
    else
        let s:job_id = result
    endif
    call s:configure(result)
endfunction

function! s:startup()
    if 0 == s:job_id
        let id = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:display_error') })
        return id
    else
        return 0
    endif
endfunction

" Shutdown
function! s:shutdown()
    if 0 > s:job_id
        augroup ctrlp-rs
            " clear all previous autocommands
            autocmd!
        augroup END

        " Give the job half a second to stop by itself
        call rpcnotify(s:job_id, 'shutdown')
        let result = jobwait(s:job_id, 500)

        " If it still didn't shut down properly, kill it
        if -1 == result
            call jobstop(s:job_id)
        endif

        " Reset the job id
        let s:job_id = 0
    endif
endfunction

" Error
function! s:display_error(id, data, event) dict
    echom 'ctrlp error: ' . join(a:data, "\n")
endfunction

" Configuration
function! s:configure(job_id)
    augroup ctrlp-rs
        " clear all previous autocommands
        autocmd!

        autocmd VimLeavePre * :call s:shutdown()
    augroup END
endfunction

function! ctrlp#search()
    call rpcnotify(s:job_id, 'search')
endfunction

function! ctrlp#shutdown()
    call rpcnotify(s:job_id, 'shutdown')
endfunction

noremap <silent> <C-P> :ctrlp#search <CR>
