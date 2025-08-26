import { mount } from '@vue/test-utils';
import { describe, it, expect } from 'vitest';
import CustomButton from '../../components/ui/CustomButton.vue';

describe('Button', () => {
  it('renders with default props', () => {
    const wrapper = mount(CustomButton, {
      slots: { default: 'Click me' },
    });

    expect(wrapper.text()).toContain('Click me');
    expect(wrapper.classes()).toContain('btn');
    expect(wrapper.classes()).toContain('btn-primary');
  });

  it('renders with different variants', () => {
    const variants = ['primary', 'secondary', 'danger', 'ghost'] as const;

    variants.forEach(variant => {
      const wrapper = mount(CustomButton, {
        props: { variant },
        slots: { default: 'Test' },
      });

      expect(wrapper.classes()).toContain(`btn-${variant}`);
    });
  });

  it('renders with different sizes', () => {
    const sizes = ['sm', 'md', 'lg'] as const;

    sizes.forEach(size => {
      const wrapper = mount(CustomButton, {
        props: { size },
        slots: { default: 'Test' },
      });

      expect(wrapper.classes()).toContain(`btn-${size}`);
    });
  });

  it('emits click event', async () => {
    const wrapper = mount(CustomButton, {
      slots: { default: 'Click me' },
    });

    await wrapper.trigger('click');

    expect(wrapper.emitted('click')).toBeTruthy();
    expect(wrapper.emitted('click')).toHaveLength(1);
  });

  it('is disabled when disabled prop is true', () => {
    const wrapper = mount(CustomButton, {
      props: { disabled: true },
      slots: { default: 'Disabled' },
    });

    expect(wrapper.attributes('disabled')).toBeDefined();
    expect(wrapper.classes()).toContain('btn-disabled');
  });

  it('renders with custom class', () => {
    const wrapper = mount(CustomButton, {
      props: { class: 'custom-class' },
      slots: { default: 'Custom' },
    });

    expect(wrapper.classes()).toContain('custom-class');
  });

  it('renders with title attribute', () => {
    const wrapper = mount(CustomButton, {
      props: { title: 'Tooltip text' },
      slots: { default: 'With tooltip' },
    });

    expect(wrapper.attributes('title')).toBe('Tooltip text');
  });
});
 