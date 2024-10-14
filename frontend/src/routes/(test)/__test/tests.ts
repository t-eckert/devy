export interface Test {
  name: string;
  path: string;
}

export interface Tests {
  components: Test[];
  markdown: Test[];
}

const tests: Tests = {
  components: [
    {
      name: 'Accordian',
      path: '/components/accordian'
    },
    {
      name: 'Avatar with Image',
      path: '/components/avatar/with-image'
    },
    {
      name: 'Menu',
      path: '/components/menu'
    }
  ],
  markdown: [
    {
      name: 'Markdown',
      path: '/markdown'
    }
  ]
};

export default tests;
