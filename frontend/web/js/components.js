/**
 * UI Components for Sanad Islamic App
 * Reusable UI components used throughout the application
 */

window.SanadComponents = {

    /**
     * Create a card component
     */
    createCard(options = {}) {
        const {
            title = '',
            content = '',
            icon = '',
            className = '',
            onClick = null
        } = options;

        const card = window.SanadUtils.dom.create('div', {
            className: `card ${className}`
        });

        if (icon) {
            const iconEl = window.SanadUtils.dom.create('div', {
                className: 'card-icon'
            }, icon);
            card.appendChild(iconEl);
        }

        if (title) {
            const titleEl = window.SanadUtils.dom.create('h3', {
                className: 'card-title'
            }, title);
            card.appendChild(titleEl);
        }

        if (content) {
            const contentEl = window.SanadUtils.dom.create('div', {
                className: 'card-content'
            });
            if (typeof content === 'string') {
                contentEl.innerHTML = content;
            } else {
                contentEl.appendChild(content);
            }
            card.appendChild(contentEl);
        }

        if (onClick) {
            card.style.cursor = 'pointer';
            window.SanadUtils.dom.on(card, 'click', onClick);
        }

        return card;
    },

    /**
     * Create a button component
     */
    createButton(options = {}) {
        const {
            text = '',
            type = 'primary',
            icon = '',
            onClick = null,
            disabled = false,
            className = ''
        } = options;

        const button = window.SanadUtils.dom.create('button', {
            className: `btn btn-${type} ${className}`,
            disabled: disabled
        });

        if (icon) {
            const iconEl = window.SanadUtils.dom.create('span', {
                className: 'btn-icon'
            }, icon);
            button.appendChild(iconEl);
        }

        if (text) {
            const textEl = window.SanadUtils.dom.create('span', {
                className: 'btn-text'
            }, text);
            button.appendChild(textEl);
        }

        if (onClick) {
            window.SanadUtils.dom.on(button, 'click', onClick);
        }

        return button;
    },

    /**
     * Create a loading spinner
     */
    createSpinner(size = 'medium') {
        const spinner = window.SanadUtils.dom.create('div', {
            className: `spinner spinner-${size}`
        });
        return spinner;
    },

    /**
     * Create a modal component
     */
    createModal(options = {}) {
        const {
            title = '',
            content = '',
            onClose = null,
            showClose = true,
            className = ''
        } = options;

        const modal = window.SanadUtils.dom.create('div', {
            className: `modal ${className}`
        });

        const modalContent = window.SanadUtils.dom.create('div', {
            className: 'modal-content'
        });

        // Header
        if (title || showClose) {
            const header = window.SanadUtils.dom.create('div', {
                className: 'modal-header'
            });

            if (title) {
                const titleEl = window.SanadUtils.dom.create('h2', {
                    className: 'modal-title'
                }, title);
                header.appendChild(titleEl);
            }

            if (showClose) {
                const closeBtn = window.SanadUtils.dom.create('button', {
                    className: 'modal-close'
                }, '×');
                window.SanadUtils.dom.on(closeBtn, 'click', () => {
                    this.closeModal(modal);
                    if (onClose) onClose();
                });
                header.appendChild(closeBtn);
            }

            modalContent.appendChild(header);
        }

        // Body
        const body = window.SanadUtils.dom.create('div', {
            className: 'modal-body'
        });
        if (typeof content === 'string') {
            body.innerHTML = content;
        } else if (content) {
            body.appendChild(content);
        }
        modalContent.appendChild(body);

        modal.appendChild(modalContent);
        return modal;
    },

    /**
     * Show modal
     */
    showModal(modal) {
        const overlay = document.getElementById('modalOverlay');
        if (overlay) {
            overlay.innerHTML = '';
            overlay.appendChild(modal);
            overlay.classList.add('active');
        }
    },

    /**
     * Close modal
     */
    closeModal(modal) {
        const overlay = document.getElementById('modalOverlay');
        if (overlay) {
            overlay.classList.remove('active');
            setTimeout(() => {
                overlay.innerHTML = '';
            }, 300);
        }
    },

    /**
     * Create a tab component
     */
    createTabs(options = {}) {
        const {
            tabs = [],
            activeTab = 0,
            onTabChange = null
        } = options;

        const container = window.SanadUtils.dom.create('div', {
            className: 'tabs-container'
        });

        const tabList = window.SanadUtils.dom.create('div', {
            className: 'tab-list'
        });

        const tabContent = window.SanadUtils.dom.create('div', {
            className: 'tab-content'
        });

        tabs.forEach((tab, index) => {
            // Tab button
            const tabBtn = window.SanadUtils.dom.create('button', {
                className: `tab-btn ${index === activeTab ? 'active' : ''}`
            }, tab.label);

            window.SanadUtils.dom.on(tabBtn, 'click', () => {
                // Update active tab
                tabList.querySelectorAll('.tab-btn').forEach(btn => btn.classList.remove('active'));
                tabBtn.classList.add('active');

                // Update content
                tabContent.querySelectorAll('.tab-pane').forEach(pane => pane.classList.remove('active'));
                tabContent.querySelector(`[data-tab="${index}"]`).classList.add('active');

                if (onTabChange) onTabChange(index);
            });

            tabList.appendChild(tabBtn);

            // Tab pane
            const pane = window.SanadUtils.dom.create('div', {
                className: `tab-pane ${index === activeTab ? 'active' : ''}`,
                'data-tab': index
            });
            if (typeof tab.content === 'string') {
                pane.innerHTML = tab.content;
            } else if (tab.content) {
                pane.appendChild(tab.content);
            }
            tabContent.appendChild(pane);
        });

        container.appendChild(tabList);
        container.appendChild(tabContent);

        return container;
    },

    /**
     * Create a list component
     */
    createList(options = {}) {
        const {
            items = [],
            type = 'simple',
            onItemClick = null
        } = options;

        const list = window.SanadUtils.dom.create('ul', {
            className: `list list-${type}`
        });

        items.forEach((item, index) => {
            const li = window.SanadUtils.dom.create('li', {
                className: 'list-item'
            });

            if (typeof item === 'string') {
                li.textContent = item;
            } else {
                if (item.icon) {
                    const icon = window.SanadUtils.dom.create('span', {
                        className: 'list-item-icon'
                    }, item.icon);
                    li.appendChild(icon);
                }
                if (item.text) {
                    const text = window.SanadUtils.dom.create('span', {
                        className: 'list-item-text'
                    }, item.text);
                    li.appendChild(text);
                }
                if (item.subtitle) {
                    const subtitle = window.SanadUtils.dom.create('span', {
                        className: 'list-item-subtitle'
                    }, item.subtitle);
                    li.appendChild(subtitle);
                }
            }

            if (onItemClick) {
                li.style.cursor = 'pointer';
                window.SanadUtils.dom.on(li, 'click', () => onItemClick(item, index));
            }

            list.appendChild(li);
        });

        return list;
    },

    /**
     * Create an input field component
     */
    createInput(options = {}) {
        const {
            type = 'text',
            placeholder = '',
            value = '',
            name = '',
            id = '',
            className = '',
            required = false,
            onInput = null,
            onChange = null
        } = options;

        const wrapper = window.SanadUtils.dom.create('div', {
            className: `input-wrapper ${className}`
        });

        const input = window.SanadUtils.dom.create('input', {
            type: type,
            placeholder: placeholder,
            value: value,
            name: name,
            id: id,
            className: 'input-field',
            required: required
        });

        if (onInput) {
            window.SanadUtils.dom.on(input, 'input', onInput);
        }

        if (onChange) {
            window.SanadUtils.dom.on(input, 'change', onChange);
        }

        wrapper.appendChild(input);
        return wrapper;
    },

    /**
     * Create a progress bar component
     */
    createProgressBar(options = {}) {
        const {
            value = 0,
            max = 100,
            showLabel = true,
            className = ''
        } = options;

        const container = window.SanadUtils.dom.create('div', {
            className: `progress-container ${className}`
        });

        const bar = window.SanadUtils.dom.create('div', {
            className: 'progress-bar'
        });

        const fill = window.SanadUtils.dom.create('div', {
            className: 'progress-fill'
        });
        fill.style.width = `${(value / max) * 100}%`;
        bar.appendChild(fill);

        container.appendChild(bar);

        if (showLabel) {
            const label = window.SanadUtils.dom.create('span', {
                className: 'progress-label'
            }, `${Math.round((value / max) * 100)}%`);
            container.appendChild(label);
        }

        return container;
    },

    /**
     * Create a badge component
     */
    createBadge(options = {}) {
        const {
            text = '',
            type = 'default',
            className = ''
        } = options;

        return window.SanadUtils.dom.create('span', {
            className: `badge badge-${type} ${className}`
        }, text);
    },

    /**
     * Create an alert component
     */
    createAlert(options = {}) {
        const {
            message = '',
            type = 'info',
            dismissible = true,
            onDismiss = null
        } = options;

        const alert = window.SanadUtils.dom.create('div', {
            className: `alert alert-${type}`
        });

        const messageEl = window.SanadUtils.dom.create('span', {
            className: 'alert-message'
        }, message);
        alert.appendChild(messageEl);

        if (dismissible) {
            const closeBtn = window.SanadUtils.dom.create('button', {
                className: 'alert-close'
            }, '×');
            window.SanadUtils.dom.on(closeBtn, 'click', () => {
                alert.remove();
                if (onDismiss) onDismiss();
            });
            alert.appendChild(closeBtn);
        }

        return alert;
    },

    /**
     * Create a tooltip component
     */
    createTooltip(element, text, position = 'top') {
        const tooltip = window.SanadUtils.dom.create('div', {
            className: `tooltip tooltip-${position}`
        }, text);

        window.SanadUtils.dom.on(element, 'mouseenter', () => {
            document.body.appendChild(tooltip);
            const rect = element.getBoundingClientRect();
            tooltip.style.left = `${rect.left + rect.width / 2}px`;
            tooltip.style.top = `${rect.top - 10}px`;
            tooltip.classList.add('visible');
        });

        window.SanadUtils.dom.on(element, 'mouseleave', () => {
            tooltip.classList.remove('visible');
            setTimeout(() => tooltip.remove(), 300);
        });

        return element;
    }
};

// Freeze the components object
Object.freeze(window.SanadComponents);
