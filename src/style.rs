pub const GLOBAL_CSS: &str = r#"
    :root {
        --bg-primary: #16161A;
        --bg-secondary: #202026;
        --bg-tertiary: #29292E;
        --bg-quaternary: #333339;

        --text-primary: #E1E1E6;
        --text-secondary: #A8A8B3;
        --text-tertiary: #7C7C8A;

        --brand-primary: #00B37E;
        --brand-secondary: #00875F;
        --brand-hover: #00D495;

        --warning: #EA8817;
        --danger: #F75A68;

        --input-bg: #202026;

        --border-color: #333339;
        --neutral-hover: #29292E;
        --active-item: #3E3E46;
    }

    .light-theme {
        --bg-primary: #FFFFFF;
        --bg-secondary: #F3F4F6;
        --bg-tertiary: #E5E7EB;
        --bg-quaternary: #D1D5DB;

        --text-primary: #111827;
        --text-secondary: #4B5563;
        --text-tertiary: #6C737F;

        --brand-primary: #2563EB;
        --brand-secondary: #1D4ED8;
        --brand-hover: #3B82F6;

        --warning: #D97706;
        --danger: #DC2626;

        --input-bg: #FFFFFF;

        --border-color: #D1D5DB;
        --neutral-hover: #E5E7EB;
        --active-item: #D1D5DB;
    }

    body {
        background-color: var(--bg-primary);
        color: var(--text-primary);
        font-family: system-ui, -apple-system, sans-serif;
        margin: 0;
        padding: 0;
        overflow: hidden;
        user-select: none;
    }

    .btn {
        padding: 10px;
        border-radius: 6px;
        border: 1px solid transparent;
        cursor: pointer;
        font-size: 13px;
        font-weight: 500;
        transition: all 0.15s ease-in-out;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .btn:active { transform: scale(0.98); }

    .btn-brand {
        background-color: var(--brand-primary);
        color: white;
    }
    .btn-brand:hover { background-color: var(--brand-hover); }
    .btn-brand:active { background-color: var(--brand-secondary); }

    .btn-ghost {
        background-color: transparent;
        color: var(--text-secondary);
    }
    .btn-ghost:hover {
        background-color: var(--neutral-hover);
        color: var(--text-primary);
    }
    .btn-ghost.active {
        background-color: var(--active-item);
        color: var(--text-primary);
        font-weight: 600;
    }

    .btn-neutral {
        background-color: var(--bg-secondary);
        color: var(--text-secondary);
        border: 1px solid var(--border-color);
        justify-content: flex-start;
    }
    .btn-neutral:hover {
        background-color: var(--neutral-hover);
        color: var(--text-primary);
        border-color: var(--text-tertiary);
    }
    .btn-neutral.active {
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
        border-color: var(--brand-primary);
    }

    .btn-warning { background-color: var(--warning); color: white; }
    .btn-warning:hover { filter: brightness(1.1); }
    .btn-warning:active { filter: brightness(0.9); }

    .btn-danger { background-color: var(--danger); color: white; }
    .btn-danger:hover { filter: brightness(1.1); }
    .btn-danger:active { filter: brightness(0.9); }


    input {
        background-color: var(--input-bg);
        border: 1px solid var(--border-color);
        color: var(--text-primary);
        padding: 10px;
        border-radius: 6px;
        outline: none;
        transition: border-color 0.2s;
    }
    input:focus {
        border-color: var(--brand-primary);
        box-shadow: 0 0 0 2px rgba(5, 150, 105, 0.1);
    }
    
    ::-webkit-scrollbar { width: 8px; }
    ::-webkit-scrollbar-track { background: transparent; }
    ::-webkit-scrollbar-thumb { background: var(--bg-quaternary); border-radius: 4px; }
    ::-webkit-scrollbar-thumb:hover { background: var(--text-tertiary); }
"#;