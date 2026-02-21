import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { createRouter, createMemoryHistory } from 'vue-router'
import App from '../App.vue'

// Stub layout components to avoid deep rendering
vi.mock('@/components/layout/AppNavbar.vue', () => ({
  default: { template: '<nav>Navbar</nav>' },
}))
vi.mock('@/components/layout/AppSidebar.vue', () => ({
  default: { template: '<aside>Sidebar</aside>' },
}))
vi.mock('@/components/layout/AppFooter.vue', () => ({
  default: { template: '<footer>Footer</footer>' },
}))

function createTestRouter(showChrome = false) {
  return createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', component: { template: '<div>Home</div>' }, meta: { showChrome } },
    ],
  })
}

describe('App', () => {
  it('renders footer always', async () => {
    const router = createTestRouter(false)
    await router.push('/')
    await router.isReady()

    const wrapper = mount(App, {
      global: { plugins: [createPinia(), router] },
    })
    expect(wrapper.text()).toContain('Footer')
  })

  it('hides navbar and sidebar when showChrome is false', async () => {
    const router = createTestRouter(false)
    await router.push('/')
    await router.isReady()

    const wrapper = mount(App, {
      global: { plugins: [createPinia(), router] },
    })
    expect(wrapper.text()).not.toContain('Navbar')
    expect(wrapper.text()).not.toContain('Sidebar')
  })

  it('shows navbar and sidebar when showChrome is true', async () => {
    const router = createTestRouter(true)
    await router.push('/')
    await router.isReady()

    const wrapper = mount(App, {
      global: { plugins: [createPinia(), router] },
    })
    expect(wrapper.text()).toContain('Navbar')
    expect(wrapper.text()).toContain('Sidebar')
  })
})
