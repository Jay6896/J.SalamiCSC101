PGDMP                       }         	   cos101_db    17.2    17.2 
    �           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                           false            �           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                           false            �           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                           false            �           1262    16388 	   cos101_db    DATABASE     �   CREATE DATABASE cos101_db WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'English_United States.1252';
    DROP DATABASE cos101_db;
                     postgres    false            �            1259    16397    company    TABLE     �   CREATE TABLE public.company (
    id integer NOT NULL,
    name text NOT NULL,
    age integer NOT NULL,
    address character(50),
    salary real,
    join_date date
);
    DROP TABLE public.company;
       public         heap r       postgres    false            �            1259    16392 
   department    TABLE     z   CREATE TABLE public.department (
    id integer NOT NULL,
    dept character(50) NOT NULL,
    emp_id integer NOT NULL
);
    DROP TABLE public.department;
       public         heap r       postgres    false            �          0    16397    company 
   TABLE DATA           L   COPY public.company (id, name, age, address, salary, join_date) FROM stdin;
    public               postgres    false    218   �	       �          0    16392 
   department 
   TABLE DATA           6   COPY public.department (id, dept, emp_id) FROM stdin;
    public               postgres    false    217   �
       [           2606    16403    company company_pkey 
   CONSTRAINT     R   ALTER TABLE ONLY public.company
    ADD CONSTRAINT company_pkey PRIMARY KEY (id);
 >   ALTER TABLE ONLY public.company DROP CONSTRAINT company_pkey;
       public                 postgres    false    218            Y           2606    16396    department department_pkey 
   CONSTRAINT     X   ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (id);
 D   ALTER TABLE ONLY public.department DROP CONSTRAINT department_pkey;
       public                 postgres    false    217            �   �   x����
�@E��|E?`�;n�c����()�
m���R���a��ù�B��Q�e3����t�l��y����wt8�9����ja�N�!Ʊ��0v�9���H�ruE��X��El����X���$���f2I#����q*��7���������̷f�%�N�Nk      �      x������ � �     